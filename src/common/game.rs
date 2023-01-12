use crate::prelude::*;

type SceneConstructor = Box<dyn FnOnce(&mut GameIO) -> Box<dyn Scene>>;
type OverlayConstructor = Box<dyn FnOnce(&mut GameIO) -> Box<dyn SceneOverlay>>;
type SetupCallback = Box<dyn FnOnce(&mut GameIO)>;

pub struct Game {
    window_config: WindowConfig,
    target_fps: u16,
    overlay_constructors: Vec<OverlayConstructor>,
    setup_callbacks: Vec<SetupCallback>,
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

    pub fn with_overlay<OverlayConstructor>(
        mut self,
        overlay_constuctor: OverlayConstructor,
    ) -> Self
    where
        OverlayConstructor: FnOnce(&mut GameIO) -> Box<dyn SceneOverlay> + 'static,
    {
        self.overlay_constructors.push(Box::new(overlay_constuctor));
        self
    }

    pub fn run<SceneConstructor>(self, scene_constructor: SceneConstructor) -> anyhow::Result<()>
    where
        SceneConstructor: FnOnce(&mut GameIO) -> Box<dyn Scene> + 'static,
    {
        let window_loop = Window::build(self.window_config)?;

        let params = WindowLoopParams {
            scene_constructor: Box::new(scene_constructor),
            target_fps: self.target_fps,
            overlay_constructors: self.overlay_constructors,
            setup_callbacks: self.setup_callbacks,
        };

        pollster::block_on(window_loop.run(params))
    }
}

pub(crate) struct WindowLoopParams {
    pub scene_constructor: SceneConstructor,
    pub target_fps: u16,
    pub overlay_constructors: Vec<OverlayConstructor>,
    pub setup_callbacks: Vec<SetupCallback>,
}
