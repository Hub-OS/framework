use crate::common::*;
use crate::graphics::PostProcess;
use crate::runtime::*;
use std::any::TypeId;
use std::pin::Pin;

pub struct Game<Loop: GameWindowLoop> {
    window_config: GameWindowConfig<Loop::PlatformApp>,
    target_fps: u16,
    pub service_constructors: Vec<ServiceConstructor>,
    overlay_constructors: Vec<(GameOverlayTarget, OverlayConstructor)>,
    setup_callbacks: Vec<SetupCallback>,
    post_process_constructors: Vec<PostProcessConstructor>,
}

impl<Loop: GameWindowLoop> Game<Loop> {
    pub fn new(title: &str, size: (u32, u32)) -> Self {
        Game {
            target_fps: 60,
            window_config: GameWindowConfig::new(title, size),
            service_constructors: Vec::new(),
            overlay_constructors: Vec::new(),
            setup_callbacks: Vec::new(),
            post_process_constructors: Vec::new(),
        }
    }

    pub fn with_borderless(mut self, value: bool) -> Self {
        self.window_config.borderless = value;
        self
    }

    pub fn with_fullscreen(mut self, value: bool) -> Self {
        self.window_config.fullscreen = value;
        self
    }

    pub fn with_resizable(mut self, value: bool) -> Self {
        self.window_config.resizable = value;
        self
    }

    pub fn with_transparent(mut self, value: bool) -> Self {
        self.window_config.transparent = value;
        self
    }

    pub fn with_always_on_top(mut self, value: bool) -> Self {
        self.window_config.always_on_top = value;
        self
    }

    pub fn with_locked_resolution(mut self, resolution: Option<(u32, u32)>) -> Self {
        self.window_config.resolution = resolution;
        self
    }

    pub fn with_target_fps(mut self, target_fps: u16) -> Self {
        self.target_fps = target_fps;
        self
    }

    pub fn with_setup<SetupCallback>(mut self, setup_callback: SetupCallback) -> Self
    where
        SetupCallback: FnOnce(&mut GameIO) + 'static,
    {
        self.setup_callbacks.push(Box::new(setup_callback));
        self
    }

    pub fn with_service<ServiceConstructor, S>(mut self, constructor: ServiceConstructor) -> Self
    where
        ServiceConstructor: FnOnce(&mut GameIO) -> S + 'static,
        S: GameService + 'static,
    {
        let constructor =
            |game_io: &mut GameIO| -> Box<dyn GameService> { Box::new(constructor(game_io)) };

        self.service_constructors.push(Box::new(constructor));
        self
    }

    pub fn with_overlay<OverlayConstructor, O>(
        mut self,
        overlay_target: GameOverlayTarget,
        constructor: OverlayConstructor,
    ) -> Self
    where
        OverlayConstructor: FnOnce(&mut GameIO) -> O + 'static,
        O: GameOverlay + 'static,
    {
        let constructor =
            |game_io: &mut GameIO| -> Box<dyn GameOverlay> { Box::new(constructor(game_io)) };

        self.overlay_constructors
            .push((overlay_target, Box::new(constructor)));
        self
    }

    pub fn with_post_process<PostProcessConstructor, P>(
        mut self,
        constructor: PostProcessConstructor,
    ) -> Self
    where
        PostProcessConstructor: FnOnce(&mut GameIO) -> P + 'static,
        P: PostProcess + 'static,
    {
        let constructor = |game_io: &mut GameIO| -> (TypeId, Box<dyn PostProcess>) {
            (TypeId::of::<P>(), Box::new(constructor(game_io)))
        };
        self.post_process_constructors.push(Box::new(constructor));
        self
    }

    pub fn with_platform_app(mut self, platform_app: Loop::PlatformApp) -> Self {
        self.window_config.platform_app = Some(platform_app);
        self
    }

    pub fn run<SceneConstructor, S>(self, scene_constructor: SceneConstructor) -> anyhow::Result<()>
    where
        SceneConstructor: FnOnce(&mut GameIO) -> S + 'static,
        S: Scene + 'static,
    {
        let window_loop = Loop::build(self.window_config)?;

        let scene_constructor =
            |game_io: &mut GameIO| -> Box<dyn Scene> { Box::new(scene_constructor(game_io)) };

        let params = GameRuntimeCoreParams {
            scene_constructor: Box::new(scene_constructor),
            target_fps: self.target_fps,
            service_constructors: self.service_constructors,
            overlay_constructors: self.overlay_constructors,
            setup_callbacks: self.setup_callbacks,
            post_process_constructors: self.post_process_constructors,
        };

        let pinned_future = Pin::from(window_loop.run(params));

        crate::async_task::block_on(pinned_future)
    }
}
