use crate::prelude::*;
use crate::{cfg_sdl, cfg_winit};
use instant::Instant;
use std::any::TypeId;

pub(crate) struct GameRuntime {
    event_buffer: Vec<WindowEvent>,
    scene_manager: SceneManager,
    frame_end: Instant,
    game_io: GameIO,
    overlays: Vec<Box<dyn SceneOverlay>>,
    post_processes: Vec<(TypeId, Box<dyn PostProcess>)>,
    post_model: TextureSourceModel,
    render_sprite: Sprite,
    render_target: RenderTarget,
    render_target_b: RenderTarget,
    camera: OrthoCamera,
}

impl GameRuntime {
    pub(crate) async fn new(window: Window, loop_params: WindowLoopParams) -> anyhow::Result<Self> {
        let window_size = window.size();
        let graphics = GraphicsContext::new(&window, window_size.x, window_size.y).await?;

        let mut game_io = GameIO::new(window, graphics);
        game_io.set_target_fps(loop_params.target_fps);

        super::default_resources::inject(&mut game_io);

        for callback in loop_params.setup_callbacks {
            callback(&mut game_io);
        }

        let initial_scene = (loop_params.scene_constructor)(&mut game_io);

        let overlays = loop_params
            .overlay_constructors
            .into_iter()
            .map(|constructor| constructor(&mut game_io))
            .collect();

        let post_processes = loop_params
            .post_process_constructors
            .into_iter()
            .map(|constructor| constructor(&mut game_io))
            .collect();

        let render_target = RenderTarget::new(&game_io, window_size);
        let render_target_b = RenderTarget::new(&game_io, window_size);
        let render_sprite = Sprite::new(&game_io, render_target.texture().clone());
        let camera = OrthoCamera::new(&game_io, window_size.as_vec2());
        let post_model = TextureSourceModel::new(&game_io, render_target.texture().clone());

        Ok(Self {
            event_buffer: Vec::new(),
            scene_manager: SceneManager::new(initial_scene),
            frame_end: Instant::now(),
            game_io,
            overlays,
            post_processes,
            post_model,
            render_sprite,
            render_target,
            render_target_b,
            camera,
        })
    }

    pub fn game_io(&self) -> &GameIO {
        &self.game_io
    }

    pub fn is_quitting(&self) -> bool {
        self.game_io.is_quitting()
    }

    cfg_sdl! {
        pub async fn sleep(&self) {
            use crate::async_task::sleep;

            let sleep_duration = self.game_io.attempted_sleep_duration();

            if !sleep_duration.is_zero() {
                sleep(sleep_duration).await;
            }
        }
    }

    cfg_winit! {
        pub fn target_sleep_instant(&self) -> Instant {
            self.frame_end + self.game_io.attempted_sleep_duration()
        }
    }

    pub fn push_event(&mut self, event: WindowEvent) {
        self.event_buffer.push(event)
    }

    pub fn tick(&mut self) {
        if self.frame_end.elapsed() < self.game_io.attempted_sleep_duration() {
            // running too fast skip tick (this issue should only occur on web)
            return;
        }

        let start_instant = Instant::now();
        let game_io = &mut self.game_io;
        game_io.set_frame_start_instant(start_instant);

        // update the previous timing with new info before updates start
        let lost_duration = start_instant - self.frame_end - game_io.attempted_sleep_duration();
        game_io.set_lost_duration(lost_duration);

        // queue a new task
        let mut events = Vec::new();
        std::mem::swap(&mut events, &mut self.event_buffer);

        // update
        game_io.handle_tasks();
        game_io.handle_events(events);

        self.scene_manager.update(game_io);

        for overlay in &mut self.overlays {
            overlay.update(game_io);
        }

        for (id, post_process) in &mut self.post_processes {
            if !game_io.graphics().internal_is_post_process_enabled(*id) {
                continue;
            }

            post_process.update(game_io);
        }

        // kick off new tasks
        game_io.handle_tasks();

        let update_instant = Instant::now();

        // draw
        let graphics = game_io.graphics();
        let device = graphics.device();

        let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("window_command_encoder"),
        });

        let resolution = game_io.window().resolution();
        self.render_target.resize(game_io, resolution);
        self.render_target_b.resize(game_io, resolution);

        let clear_color = graphics.clear_color();
        self.render_target.set_clear_color(clear_color);
        self.render_target_b.set_clear_color(clear_color);

        // draw scene
        self.scene_manager.draw(
            game_io,
            &mut encoder,
            &mut self.render_target,
            &mut self.render_target_b,
        );

        // draw overlays, set clear color to None to recycle previous render
        self.render_target.set_clear_color(None);
        let mut render_pass = RenderPass::new(&mut encoder, &self.render_target);

        for overlay in &mut self.overlays {
            overlay.draw(game_io, &mut render_pass);
        }

        render_pass.flush();
        self.render_target.set_clear_color(clear_color);

        // post processing
        for (id, post_process) in &mut self.post_processes {
            if !game_io.graphics().internal_is_post_process_enabled(*id) {
                continue;
            }

            // set the texture for the post model to the latest texture
            self.post_model
                .set_texture(self.render_target.texture().clone());

            // swap primary target
            std::mem::swap(&mut self.render_target, &mut self.render_target_b);

            let render_pass = RenderPass::new(&mut encoder, &self.render_target);
            post_process.draw(game_io, render_pass, &self.post_model);
        }

        // update camera
        let graphics = game_io.graphics();
        let window = game_io.window();

        if window.has_locked_resolution() {
            self.camera.resize(window.resolution().as_vec2());
            self.camera.scale_to(graphics.surface_size().as_vec2());
        } else {
            self.camera.resize(window.resolution().as_vec2());
        }

        // render to window
        let buffer_aquire_start = Instant::now();
        let mut buffer_aquire_end = buffer_aquire_start;

        if let Ok(frame) = graphics.surface().get_current_texture() {
            buffer_aquire_end = Instant::now();

            let texture = &frame.texture;
            let view = texture.create_view(&wgpu::TextureViewDescriptor::default());
            let texture_size = graphics.surface_size();

            let mut window_target = RenderTarget::from_view(view, texture_size);
            window_target.set_clear_color(graphics.clear_color());

            let mut render_pass = RenderPass::new(&mut encoder, &window_target);

            // render as a sprite
            self.render_sprite
                .set_texture(self.render_target.texture().clone());
            self.render_sprite
                .set_origin(self.render_sprite.size() * 0.5);

            let uniforms = [self.camera.as_binding()];
            let mut sprite_queue = SpriteQueue::new_with_default_pipeline(game_io, uniforms);
            sprite_queue.draw_sprite(&self.render_sprite);

            render_pass.consume_queue(sprite_queue);
            render_pass.flush();

            let queue = graphics.queue();
            queue.submit([encoder.finish()]);
            frame.present();
        }

        let end_instant = Instant::now();
        let draw_duration = end_instant - update_instant;

        // tracking time spent
        game_io.set_update_duration(update_instant - start_instant);
        game_io.set_draw_duration(draw_duration);
        game_io.set_frame_duration(end_instant - start_instant);
        game_io.set_buffer_aquire_duration(buffer_aquire_end - buffer_aquire_start);
        game_io.update_sleep_duration();

        self.frame_end = end_instant;
    }
}
