use crate::prelude::*;
use std::any::TypeId;

type SceneConstructor = Box<dyn FnOnce(&mut GameIO) -> Box<dyn Scene>>;
type OverlayConstructor = Box<dyn FnOnce(&mut GameIO) -> Box<dyn SceneOverlay>>;
type PostProcessConstructor = Box<dyn FnOnce(&mut GameIO) -> (TypeId, Box<dyn PostProcess>)>;
type SetupCallback = Box<dyn FnOnce(&mut GameIO)>;

pub struct Game {
    window_config: WindowConfig,
    target_fps: u16,
    overlay_constructors: Vec<OverlayConstructor>,
    setup_callbacks: Vec<SetupCallback>,
    post_process_constructors: Vec<PostProcessConstructor>,
}

impl Game {
    pub fn new(title: &str, size: (u32, u32)) -> Game {
        Game {
            target_fps: 60,
            window_config: WindowConfig {
                title: String::from(title),
                size,
                ..Default::default()
            },
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

    pub fn with_resolution(mut self, resolution: (u32, u32)) -> Self {
        self.window_config.resolution = Some(resolution);
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

    pub fn with_overlay<OverlayConstructor, O>(mut self, constructor: OverlayConstructor) -> Self
    where
        OverlayConstructor: FnOnce(&mut GameIO) -> O + 'static,
        O: SceneOverlay + 'static,
    {
        let constructor =
            |game_io: &mut GameIO| -> Box<dyn SceneOverlay> { Box::new(constructor(game_io)) };

        self.overlay_constructors.push(Box::new(constructor));
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

    pub fn run<SceneConstructor, S>(self, scene_constructor: SceneConstructor) -> anyhow::Result<()>
    where
        SceneConstructor: FnOnce(&mut GameIO) -> S + 'static,
        S: Scene + 'static,
    {
        let window_loop = Window::build(self.window_config)?;

        let scene_constructor =
            |game_io: &mut GameIO| -> Box<dyn Scene> { Box::new(scene_constructor(game_io)) };

        let params = WindowLoopParams {
            scene_constructor: Box::new(scene_constructor),
            target_fps: self.target_fps,
            overlay_constructors: self.overlay_constructors,
            setup_callbacks: self.setup_callbacks,
            post_process_constructors: self.post_process_constructors,
        };

        pollster::block_on(window_loop.run(params))
    }
}

pub(crate) struct WindowLoopParams {
    pub scene_constructor: SceneConstructor,
    pub target_fps: u16,
    pub overlay_constructors: Vec<OverlayConstructor>,
    pub setup_callbacks: Vec<SetupCallback>,
    pub post_process_constructors: Vec<PostProcessConstructor>,
}
