use crate::window_handle::AndroidWindowHandle;
use crate::AndroidPlatformApp;
use framework_core::common::GameWindow;
use framework_core::graphics::{wgpu, Color, GraphicsContext, HasGraphicsContext, RenderTarget};
use framework_core::runtime::{GameWindowConfig, GameWindowLifecycle};
use math::{IVec2, Rect, UVec2};

pub(crate) struct AndroidGameWindow {
    pub(crate) app: AndroidPlatformApp,
    graphics: GraphicsContext,
    surface: wgpu::Surface<'static>,
    surface_config: wgpu::SurfaceConfiguration,
    surface_texture: Option<wgpu::SurfaceTexture>,
    position: IVec2,
    size: UVec2,
    resolution: UVec2,
    locked_resolution: bool,
    integer_scaling: bool,
    fullscreen: bool,
    clear_color: Option<Color>,
}

impl AndroidGameWindow {
    pub(crate) async fn new(
        mut window_config: GameWindowConfig<AndroidPlatformApp>,
    ) -> anyhow::Result<Self> {
        let app = window_config.platform_app.take().unwrap();
        let window = app.native_window().unwrap();
        let window_handle = AndroidWindowHandle::from(window.clone());

        let wgpu_instance = wgpu::Instance::new(&wgpu::InstanceDescriptor {
            backends: wgpu::Backends::from_env().unwrap_or(wgpu::Backends::all()),
            flags: wgpu::InstanceFlags::empty(),
            memory_budget_thresholds: wgpu::MemoryBudgetThresholds::default(),
            backend_options: Default::default(),
        });
        let surface = wgpu_instance.create_surface(window_handle).unwrap();
        let mut graphics = GraphicsContext::new(wgpu_instance, Some(&surface)).await?;

        let adapter = graphics.adapter();
        let device = graphics.device();

        let window_size = window_config.size;
        let width = window_size.x;
        let height = window_size.y;

        let mut surface_config = surface
            .get_default_config(adapter, width, height)
            .expect("Surface unsupported by adapter");

        surface_config.present_mode = wgpu::PresentMode::AutoVsync;
        surface.configure(device, &surface_config);
        graphics.set_default_texture_format(surface_config.format);

        Ok(Self {
            app,
            graphics,
            surface,
            surface_config,
            surface_texture: None,
            position: Default::default(),
            size: window_config.size,
            resolution: window_config.resolution.unwrap_or(window_config.size),
            locked_resolution: window_config.resolution.is_some(),
            integer_scaling: window_config.integer_scaling,
            fullscreen: false,
            clear_color: Some(Color::TRANSPARENT),
        })
    }
}

impl HasGraphicsContext for AndroidGameWindow {
    fn graphics(&self) -> &GraphicsContext {
        &self.graphics
    }
}

impl GameWindowLifecycle for AndroidGameWindow {
    fn rebuild_surface(&mut self) {
        let Some(window) = self.app.native_window() else {
            return;
        };

        let graphics = self.graphics();
        let instance = graphics.wgpu_instance();
        let device = graphics.device();

        let window_handle = AndroidWindowHandle::from(window.clone());

        if let Ok(surface) = instance.create_surface(window_handle) {
            surface.configure(device, &self.surface_config);
            self.surface = surface;
        }
    }

    fn acquire_render_target(&mut self) -> Option<RenderTarget> {
        let surface_texture = self.surface.get_current_texture().ok()?;
        let texture = &surface_texture.texture;

        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());

        self.surface_texture = Some(surface_texture);

        Some(RenderTarget::from_view(view))
    }

    fn present_frame(&mut self, _render_target: RenderTarget) {
        if let Some(surface_texture) = self.surface_texture.take() {
            surface_texture.present();
        }
    }

    fn moved(&mut self, position: IVec2) {
        self.position = position;
    }

    fn resized(&mut self, size: UVec2) {
        self.size = size;

        if !self.locked_resolution {
            self.resolution = size;
        }

        self.surface_config.width = size.x.max(1);
        self.surface_config.height = size.y.max(1);

        let device = self.graphics().device();
        self.surface.configure(device, &self.surface_config);
    }

    fn set_accepting_text_input(&mut self, accept: bool) {
        if accept {
            android::util::show_ime(&self.app);
        } else {
            android::util::hide_ime(&self.app);
        }
    }

    fn set_ime_cursor_area(&mut self, _rect: Rect) {}
}

impl GameWindow for AndroidGameWindow {
    fn position(&self) -> IVec2 {
        self.position
    }

    fn set_position(&mut self, _position: IVec2) {
        log::warn!("AndroidGameWindow::set_position() is unimplemented");
    }

    fn fullscreen(&self) -> bool {
        self.fullscreen
    }

    fn set_fullscreen(&mut self, fullscreen: bool) {
        if fullscreen {
            android::util::hide_system_bars(&self.app);
        } else {
            android::util::show_system_bars(&self.app);
        }

        self.fullscreen = fullscreen;
    }

    fn size(&self) -> UVec2 {
        self.size
    }

    fn request_size(&mut self, _size: UVec2) {
        log::warn!("AndroidGameWindow::request_size() is unimplemented");
    }

    fn has_locked_resolution(&self) -> bool {
        self.locked_resolution
    }

    fn lock_resolution(&mut self, resolution: UVec2) {
        self.resolution = resolution;
        self.locked_resolution = true;
    }

    fn unlock_resolution(&mut self) {
        self.resolution = self.size;
        self.locked_resolution = false;
    }

    fn resolution(&self) -> UVec2 {
        self.resolution
    }

    fn integer_scaling(&self) -> bool {
        self.integer_scaling
    }

    fn set_integer_scaling(&mut self, value: bool) {
        self.integer_scaling = value;
    }

    fn set_title(&mut self, _title: &str) {
        log::warn!("AndroidGameWindow::set_title() is unimplemented");
    }

    fn clear_color(&self) -> Option<Color> {
        self.clear_color
    }

    fn set_clear_color(&mut self, color: Option<Color>) {
        self.clear_color = color
    }

    fn vsync_enabled(&self) -> bool {
        self.surface_config.present_mode == wgpu::PresentMode::AutoVsync
    }

    fn set_vsync_enabled(&mut self, enabled: bool) {
        self.surface_config.present_mode = if enabled {
            wgpu::PresentMode::AutoVsync
        } else {
            wgpu::PresentMode::AutoNoVsync
        };

        let device = self.graphics.device();
        self.surface.configure(device, &self.surface_config);
    }

    fn ime_height(&self) -> i32 {
        android::util::get_ime_height(&self.app)
    }
}
