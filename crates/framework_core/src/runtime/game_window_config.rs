use math::UVec2;

pub struct GameWindowConfig<PlatformApp> {
    pub title: String,
    pub size: UVec2,
    pub resolution: Option<UVec2>,
    pub borderless: bool,
    pub fullscreen: bool,
    pub resizable: bool,
    pub always_on_top: bool,
    pub transparent: bool,
    pub platform_app: Option<PlatformApp>,
}

impl<T> GameWindowConfig<T> {
    pub(crate) fn new(title: &str, size: (u32, u32)) -> Self {
        Self {
            title: title.into(),
            size: size.into(),
            resolution: None,
            borderless: false,
            fullscreen: false,
            resizable: false,
            always_on_top: false,
            transparent: false,
            platform_app: None,
        }
    }
}
