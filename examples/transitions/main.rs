use framework::logging::*;
use framework::prelude::*;

mod push_transition;
use push_transition::*;

fn main() -> anyhow::Result<()> {
    default_logger::init!();

    let game = Game::<WinitGameLoop>::new("Transitions", (800, 600));

    game.run(|game_io| ExampleScene::new(game_io, 0))
}

// transition

// example scene

struct ExampleScene {
    camera: OrthoCamera,
    sprite: Sprite,
    depth: usize,
    focused: bool,
    next_scene: NextScene,
}

impl ExampleScene {
    fn new(game_io: &mut GameIO, depth: usize) -> Self {
        let mut camera = OrthoCamera::new(game_io, Vec2::new(800.0, 600.0));
        camera.set_inverted_y(true);

        let texture_bytes = match depth {
            0 => &include_bytes!("A.png")[..],
            1 => &include_bytes!("B.png")[..],
            _ => &include_bytes!("C.png")[..],
        };

        let texture = Texture::load_from_memory(game_io, texture_bytes).unwrap();

        let mut sprite = Sprite::new(game_io, texture);
        sprite.set_origin(sprite.size() * 0.5);
        sprite.set_scale(Vec2::ONE * 10.0);

        Self {
            camera,
            sprite,
            depth,
            focused: true,
            next_scene: NextScene::None,
        }
    }
}

impl Scene for ExampleScene {
    fn next_scene(&mut self) -> &mut NextScene {
        &mut self.next_scene
    }

    fn enter(&mut self, _: &mut GameIO) {
        self.focused = true;
    }

    fn update(&mut self, game_io: &mut GameIO) {
        if !self.focused {
            // block setting next_scene when we're not in focus
            return;
        }

        let just_pressed_space = game_io.input().was_key_just_pressed(Key::Space);
        let just_pressed_shift = game_io.input().was_key_just_pressed(Key::LShift);

        // handle new scene creation
        if self.depth < 2 && just_pressed_space {
            let direction = match self.depth {
                0 => TransitionDirection::Right,
                1 => TransitionDirection::Up,
                _ => unreachable!(),
            };

            let scene = ExampleScene::new(game_io, self.depth + 1);
            let transition = PushTransition::new(game_io, direction, Duration::from_secs_f32(0.5));

            self.next_scene = NextScene::new_push(scene).with_transition(transition);
        }

        // handle returning to a previous scene
        if self.depth > 0 && self.next_scene.is_none() && just_pressed_shift {
            let direction = match self.depth {
                0 => unreachable!(),
                1 => TransitionDirection::Left,
                _ => TransitionDirection::Down,
            };

            let transition = PushTransition::new(game_io, direction, Duration::from_secs_f32(0.5));
            self.next_scene = NextScene::new_pop().with_transition(transition);
        }

        if self.next_scene.is_some() {
            // we've set the next scene, so we're no longer in focus
            self.focused = false;
        }
    }

    fn draw(&mut self, game_io: &mut GameIO, render_pass: &mut RenderPass) {
        self.camera.scale_to_window(game_io.window());

        let uniforms = [self.camera.as_binding()];
        let mut render_queue =
            SpriteQueue::new_with_default_pipeline(game_io, uniforms).with_inverted_y(true);

        render_queue.draw_sprite(&self.sprite);

        render_pass.consume_queue(render_queue);
    }
}
