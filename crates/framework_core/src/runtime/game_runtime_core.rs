use crate::common::*;
use crate::graphics::*;
use crate::runtime::*;
use math::{Instant, Vec2};
use std::any::TypeId;

pub type SceneConstructor = Box<dyn FnOnce(&mut GameIO) -> Box<dyn Scene>>;
pub type ServiceConstructor = Box<dyn FnOnce(&mut GameIO) -> Box<dyn GameService>>;
pub type OverlayConstructor = Box<dyn FnOnce(&mut GameIO) -> Box<dyn GameOverlay>>;
pub type PostProcessConstructor = Box<dyn FnOnce(&mut GameIO) -> (TypeId, Box<dyn PostProcess>)>;
pub type SetupCallback = Box<dyn FnOnce(&mut GameIO)>;

pub struct GameRuntimeCoreParams {
    pub scene_constructor: SceneConstructor,
    pub target_fps: u16,
    pub service_constructors: Vec<ServiceConstructor>,
    pub overlay_constructors: Vec<(GameOverlayTarget, OverlayConstructor)>,
    pub setup_callbacks: Vec<SetupCallback>,
    pub post_process_constructors: Vec<PostProcessConstructor>,
}

pub struct GameRuntimeCore {
    event_buffer: Vec<GameWindowEvent>,
    scene_manager: SceneManager,
    frame_end: Instant,
    game_io: GameIO,
    services: Vec<Box<dyn GameService>>,
    render_overlays: Vec<Box<dyn GameOverlay>>,
    window_overlays: Vec<Box<dyn GameOverlay>>,
    post_processes: Vec<(TypeId, Box<dyn PostProcess>)>,
    post_model: TextureSourceModel,
    render_sprite: Sprite,
    render_target: RenderTarget,
    render_target_b: RenderTarget,
    camera: OrthoCamera,
}

impl GameRuntimeCore {
    pub fn new(
        window: Box<dyn GameWindowLifecycle>,
        params: GameRuntimeCoreParams,
    ) -> anyhow::Result<Self> {
        let window_size = window.size();

        let mut game_io = GameIO::new(window);
        game_io.set_target_fps(params.target_fps);

        crate::common::default_resources::inject(&mut game_io);

        for callback in params.setup_callbacks {
            callback(&mut game_io);
        }

        // services
        let mut services = Vec::new();

        for constructor in params.service_constructors {
            services.push(constructor(&mut game_io));
        }

        // initial scene
        let initial_scene = (params.scene_constructor)(&mut game_io);

        // overlays
        let mut render_overlays = Vec::new();
        let mut window_overlays = Vec::new();

        for (target, constructor) in params.overlay_constructors {
            let overlay = constructor(&mut game_io);

            match target {
                GameOverlayTarget::Render => render_overlays.push(overlay),
                GameOverlayTarget::Window => window_overlays.push(overlay),
            }
        }

        // post processes
        let post_processes = params
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
            scene_manager: SceneManager::new(&mut game_io, initial_scene),
            frame_end: Instant::now(),
            game_io,
            services,
            render_overlays,
            window_overlays,
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

    pub fn game_io_mut(&mut self) -> &mut GameIO {
        &mut self.game_io
    }

    pub fn set_suspended(&mut self, suspended: bool) {
        self.game_io.set_suspended(suspended);
    }

    pub fn quitting(&self) -> bool {
        self.game_io.quitting()
    }

    pub async fn sleep(&self) {
        use crate::async_task::sleep;

        let sleep_duration = self.game_io.target_sleep_duration();

        if !sleep_duration.is_zero() {
            sleep(sleep_duration).await;
        }
    }

    pub fn target_wake_instant(&self) -> Instant {
        self.frame_end + self.game_io.target_sleep_duration()
    }

    pub fn push_event(&mut self, event: GameWindowEvent) {
        self.event_buffer.push(event)
    }

    pub fn tick(&mut self) {
        if self.frame_end.elapsed() < self.game_io.target_sleep_duration() {
            // running too fast skip tick (this issue should only occur on web)
            return;
        }

        let start_instant = Instant::now();
        let game_io = &mut self.game_io;
        game_io.set_frame_start_instant(start_instant);

        // update the previous timing with new info before updates start
        let lost_duration = start_instant - self.frame_end - game_io.target_sleep_duration();
        game_io.set_lost_duration(lost_duration);

        // queue a new task
        let mut events = Vec::new();
        std::mem::swap(&mut events, &mut self.event_buffer);

        // update
        game_io.handle_tasks();
        game_io.handle_events(events);

        // pre_updates
        for service in &mut self.services {
            service.pre_update(game_io);
        }

        for overlay in &mut self.window_overlays {
            overlay.pre_update(game_io);
        }

        for overlay in &mut self.render_overlays {
            overlay.pre_update(game_io);
        }

        // scene update
        self.scene_manager.update(game_io);

        // post_updates
        for overlay in &mut self.render_overlays {
            overlay.post_update(game_io);
        }

        for (id, post_process) in &mut self.post_processes {
            if !game_io.internal_is_post_process_enabled(*id) {
                continue;
            }

            post_process.update(game_io);
        }

        for overlay in &mut self.window_overlays {
            overlay.post_update(game_io);
        }

        for service in &mut self.services {
            service.post_update(game_io);
        }

        // kick off new tasks
        game_io.handle_tasks();

        let update_instant = Instant::now();

        // draw
        let window = game_io.window();
        let graphics = game_io.graphics();
        let device = graphics.device();

        let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("window_command_encoder"),
        });

        let resolution = window.resolution();
        let clear_color = window.clear_color();

        self.render_target.resize(game_io, resolution);
        self.render_target_b.resize(game_io, resolution);

        self.render_target.set_clear_color(clear_color);
        self.render_target_b.set_clear_color(clear_color);

        // draw scene
        self.scene_manager.draw(
            game_io,
            &mut encoder,
            &mut self.render_target,
            &mut self.render_target_b,
        );

        // draw overlays
        if !self.render_overlays.is_empty() {
            // set clear color to None to recycle previous render
            self.render_target.set_clear_color(None);
            let mut render_pass = RenderPass::new(&mut encoder, &self.render_target);

            for overlay in &mut self.render_overlays {
                overlay.draw(game_io, &mut render_pass);
            }

            render_pass.flush();
            self.render_target.set_clear_color(clear_color);
        }

        // post processing
        for (id, post_process) in &mut self.post_processes {
            if !game_io.internal_is_post_process_enabled(*id) {
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
        let window = game_io.window();
        let window_size = window.size().as_vec2();
        let render_scale = window.render_scale();
        let inverted_render_scale = 1.0 / render_scale;
        self.camera.resize(window_size);
        self.camera.set_scale(Vec2::splat(render_scale));
        // extra positioning math to avoid fractional placement with integer scaling
        self.camera
            .set_position((window_size * 0.5 * inverted_render_scale).extend(0.0));

        // render to window
        let buffer_aquire_start = Instant::now();
        let mut buffer_aquire_end = buffer_aquire_start;
        let window = game_io.window_mut();

        if let Some(target) = window.acquire_render_target() {
            buffer_aquire_end = Instant::now();

            let mut render_pass = RenderPass::new(&mut encoder, &target);

            // render as a sprite
            self.render_sprite
                .set_texture(self.render_target.texture().clone());
            self.render_sprite.set_origin(Vec2::ZERO);
            // extra positioning math to avoid fractional placement with integer scaling
            self.render_sprite
                .set_position(window.render_offset() * inverted_render_scale);

            let uniforms = [self.camera.as_binding()];
            let mut sprite_queue = SpriteQueue::new_with_default_pipeline(game_io, uniforms);
            sprite_queue.draw_sprite(&self.render_sprite);

            render_pass.consume_queue(sprite_queue);

            for overlay in &mut self.window_overlays {
                overlay.draw(game_io, &mut render_pass);
            }

            render_pass.flush();

            let queue = game_io.graphics().queue();
            queue.submit([encoder.finish()]);

            game_io.window_mut().present_frame(target);
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
