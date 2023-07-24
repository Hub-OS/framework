use crate::prelude::*;
use slotmap::DefaultKey as SceneIndex;
use slotmap::SlotMap;
use wgpu::CommandEncoder;

type SceneArena = SlotMap<SceneIndex, Box<dyn Scene>>;

struct TransitionTracker {
    transition: Box<dyn Transition>,
    from_index: SceneIndex,
    to_index: SceneIndex,
    delete_indices: Vec<SceneIndex>,
}

impl std::fmt::Debug for TransitionTracker {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TransitionTracker")
            .field("from_index", &self.from_index)
            .field("to_index", &self.to_index)
            .field("delete_indices", &self.delete_indices)
            .finish()
    }
}

impl TransitionTracker {
    fn unwind_iter(
        final_index: SceneIndex,
        trackers: &[TransitionTracker],
    ) -> impl Iterator<Item = &TransitionTracker> {
        let mut to_index = final_index;

        std::iter::from_fn(move || {
            let tracker = trackers
                .iter()
                .find(|tracker| tracker.to_index == to_index)?;

            to_index = tracker.from_index;

            Some(tracker)
        })
    }

    fn unwind_indices(
        final_index: SceneIndex,
        trackers: &[TransitionTracker],
    ) -> impl Iterator<Item = usize> + '_ {
        let mut to_index = final_index;

        std::iter::from_fn(move || {
            let (index, tracker) = trackers
                .iter()
                .enumerate()
                .find(|(_, tracker)| tracker.to_index == to_index)?;

            to_index = tracker.from_index;

            Some(index)
        })
    }
}

pub(crate) struct SceneManager {
    final_indices: Vec<SceneIndex>,
    scenes: SceneArena,
    transitions: Vec<TransitionTracker>,
}

impl SceneManager {
    pub(crate) fn new(initial_scene: Box<dyn Scene>) -> Self {
        let mut scenes = SceneArena::new();
        let top_index = scenes.insert(initial_scene);

        Self {
            final_indices: vec![top_index],
            scenes,
            transitions: Vec::new(),
        }
    }

    fn top_index_mut(&mut self) -> &mut SceneIndex {
        self.final_indices.last_mut().unwrap()
    }

    fn cleanup_transitions(&mut self) {
        let mut pending_removal = Vec::new();

        for (index, tracker) in self.transitions.iter().enumerate() {
            if tracker.transition.is_complete() {
                pending_removal.push(index);
            }
        }

        if pending_removal.is_empty() {
            return;
        }

        // update links
        let mut links_unresolved = true;
        let scenes_pending_removal: Vec<_> = pending_removal
            .iter()
            .map(|tracker_index| {
                let tracker = &self.transitions[*tracker_index];
                (tracker.to_index, tracker.delete_indices.clone())
            })
            .collect();

        while links_unresolved {
            links_unresolved = false;

            for &(to_index, ref delete_indices) in &scenes_pending_removal {
                for tracker in &mut self.transitions {
                    if !delete_indices.contains(&tracker.to_index) {
                        continue;
                    }

                    tracker.to_index = to_index;
                    links_unresolved = true;
                }
            }
        }

        // delete
        for transition_index in pending_removal.into_iter().rev() {
            let tracker = self.transitions.remove(transition_index);

            // delete dead scenes
            for scene_index in tracker.delete_indices {
                // todo: call scene.exit(game_io)?
                self.scenes.remove(scene_index);
            }
        }
    }

    pub(crate) fn update(&mut self, game_io: &mut GameIO) {
        self.cleanup_transitions();

        let top_index = *self.top_index_mut();

        // update scenes visible from transitions
        let visible_transition_iter = TransitionTracker::unwind_iter(top_index, &self.transitions);

        // reset transitioning state
        game_io.set_transitioning(false);

        for tracker in visible_transition_iter {
            // update transitioning state
            game_io.set_transitioning(true);

            // update scene
            self.scenes[tracker.from_index].update(game_io);
        }

        // update top scene
        self.scenes[top_index].update(game_io);

        while self.handle_scene_request(game_io) {}
    }

    fn handle_scene_request(&mut self, game_io: &mut GameIO) -> bool {
        let top_index = *self.top_index_mut();
        let top_scene = &mut self.scenes[top_index];

        let next_scene = top_scene.next_scene().take();

        match next_scene {
            NextScene::None => false,
            NextScene::Push {
                mut scene,
                transition,
            } => {
                scene.enter(game_io);
                self.push_scene(scene, transition);
                true
            }
            NextScene::Swap {
                mut scene,
                transition,
            } => {
                scene.enter(game_io);
                self.swap_scene(scene, transition);
                true
            }
            NextScene::PopSwap {
                mut scene,
                transition,
            } => {
                scene.enter(game_io);
                self.pop_swap_scene(scene, transition);
                true
            }
            NextScene::Pop { transition } => {
                self.pop_scene(transition);

                let top_index = *self.top_index_mut();
                self.scenes[top_index].enter(game_io);
                true
            }
        }
    }

    fn push_scene(&mut self, scene: Box<dyn Scene>, transition: Option<Box<dyn Transition>>) {
        let top_index = self.top_index_mut();

        let from_index = *top_index;
        let to_index = self.scenes.insert(scene);

        // push to_index
        self.final_indices.push(to_index);

        if let Some(transition) = transition {
            self.transitions.push(TransitionTracker {
                transition,
                from_index,
                to_index,
                delete_indices: Vec::new(),
            });
        }
    }

    fn swap_scene(&mut self, scene: Box<dyn Scene>, transition: Option<Box<dyn Transition>>) {
        let to_index = self.scenes.insert(scene);

        let top_index = self.top_index_mut();
        let from_index = *top_index;

        // swap top index
        *top_index = to_index;

        if let Some(transition) = transition {
            self.transitions.push(TransitionTracker {
                transition,
                from_index,
                to_index,
                delete_indices: vec![from_index],
            });
        }
    }

    fn pop_swap_scene(&mut self, scene: Box<dyn Scene>, transition: Option<Box<dyn Transition>>) {
        if self.final_indices.len() == 1 {
            log::error!("No scene to pop into");
            return;
        }

        let from_index = *self.top_index_mut();
        let to_index = self.scenes.insert(scene);

        // pop then swap
        self.final_indices.pop();
        let top_index = self.top_index_mut();
        let swapped_index = *top_index;
        *top_index = to_index;

        if let Some(transition) = transition {
            self.transitions.push(TransitionTracker {
                transition,
                from_index,
                to_index,
                delete_indices: vec![swapped_index, from_index],
            });
        }
    }

    fn pop_scene(&mut self, transition: Option<Box<dyn Transition>>) {
        if self.final_indices.len() == 1 {
            log::error!("No scene to pop into");
            return;
        }

        // pop
        let from_index = self.final_indices.pop().unwrap();
        let to_index = *self.top_index_mut();

        if let Some(transition) = transition {
            self.transitions.push(TransitionTracker {
                transition,
                from_index,
                to_index,
                delete_indices: vec![from_index],
            });
        }
    }

    pub(crate) fn draw(
        &mut self,
        game_io: &mut GameIO,
        encoder: &mut CommandEncoder,
        render_target: &mut RenderTarget,
        render_target_b: &mut RenderTarget,
    ) {
        let top_index = *self.top_index_mut();
        let tracker_indices: Vec<_> =
            TransitionTracker::unwind_indices(top_index, &self.transitions).collect();

        // draw the top scene
        let mut render_pass = RenderPass::new(encoder, render_target);
        self.scenes[top_index].draw(game_io, &mut render_pass);
        render_pass.flush();

        if tracker_indices.is_empty() {
            // only needed to draw one scene
            return;
        }

        let mut model = TextureSourceModel::new(game_io, render_target.texture().clone());

        for tracker_index in tracker_indices {
            let tracker = &mut self.transitions[tracker_index];

            // swap the target
            std::mem::swap(render_target, render_target_b);

            let mut render_pass = RenderPass::new(encoder, render_target);

            tracker.transition.draw(
                game_io,
                &mut render_pass,
                &mut |game_io, render_pass| {
                    let scene = &mut self.scenes[tracker.from_index];
                    scene.draw(game_io, render_pass);
                },
                &mut |game_io, render_pass| {
                    let copy_pipeline = game_io.resource::<CopyPipeline>().unwrap();
                    let mut queue = RenderQueue::new(game_io, copy_pipeline, []);
                    queue.draw_model(&model);
                    render_pass.consume_queue(queue);
                },
            );

            render_pass.flush();

            // update model texture for the next pass
            model.set_texture(render_target.texture().clone());
        }

        // render_target has the final render for the scene manager
    }
}
