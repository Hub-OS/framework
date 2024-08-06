use framework_core::graphics::{wgpu, Color, GraphicsContext, HasGraphicsContext, RenderTarget};
use framework_core::runtime::GameWindowConfig;
use framework_core::{common::GameWindow, runtime::GameWindowLifecycle};
use math::*;

pub struct Sdl2GameWindow {
    window: sdl2::video::Window,
    graphics: GraphicsContext,
    surface: wgpu::Surface<'static>,
    surface_config: wgpu::SurfaceConfiguration,
    surface_texture: Option<wgpu::SurfaceTexture>,
    position: IVec2,
    size: UVec2,
    resolution: UVec2,
    locked_resolution: bool,
    integer_scaling: bool,
    clear_color: Option<Color>,
}

impl Sdl2GameWindow {
    pub(crate) async fn from_window_and_config(
        window: sdl2::video::Window,
        window_config: GameWindowConfig<()>,
    ) -> anyhow::Result<Self> {
        let position = window.position().into();
        let size = window.size().into();

        let wgpu_instance = wgpu::Instance::default();

        let surface_target = unsafe { wgpu::SurfaceTargetUnsafe::from_window(&window).unwrap() };
        let surface = unsafe { wgpu_instance.create_surface_unsafe(surface_target).unwrap() };
        let mut graphics = GraphicsContext::new(wgpu_instance, Some(&surface)).await?;

        let adapter = graphics.adapter();
        let device = graphics.device();

        let window_size = window.size();
        let width = window_size.0;
        let height = window_size.1;

        let mut surface_config = surface
            .get_default_config(adapter, width, height)
            .expect("Surface unsupported by adapter");

        surface_config.present_mode = wgpu::PresentMode::AutoVsync;
        surface.configure(device, &surface_config);
        graphics.set_default_texture_format(surface_config.format);

        Ok(Self {
            window,
            graphics,
            surface,
            surface_config,
            surface_texture: None,
            position,
            size,
            locked_resolution: window_config.resolution.is_some(),
            integer_scaling: window_config.integer_scaling,
            resolution: window_config.resolution.unwrap_or(size),
            clear_color: Some(Color::TRANSPARENT),
        })
    }

    pub(crate) fn id(&self) -> u32 {
        self.window.id()
    }
}

impl HasGraphicsContext for Sdl2GameWindow {
    fn graphics(&self) -> &GraphicsContext {
        &self.graphics
    }
}

impl GameWindowLifecycle for Sdl2GameWindow {
    fn rebuild_surface(&mut self) {
        let graphics = self.graphics();
        let instance = graphics.wgpu_instance();
        let device = graphics.device();

        let Ok(surface_target) = (unsafe { wgpu::SurfaceTargetUnsafe::from_window(&self.window) })
        else {
            return;
        };

        if let Ok(surface) = unsafe { instance.create_surface_unsafe(surface_target) } {
            surface.configure(device, &self.surface_config);
            self.surface = surface;
        }
    }

    fn acquire_render_target(&mut self) -> Option<RenderTarget> {
        let surface_texture = self.surface.get_current_texture().ok()?;
        let texture = &surface_texture.texture;

        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());
        let texture_size = self.size();

        self.surface_texture = Some(surface_texture);

        Some(RenderTarget::from_view(view, texture_size))
    }

    fn present_frame(&mut self, _target: RenderTarget) {
        if let Some(surface_texture) = self.surface_texture.take() {
            surface_texture.present();
        }
    }

    fn moved(&mut self, position: IVec2) {
        self.position = position;
    }

    fn resized(&mut self, size: UVec2) {
        self.size = size;

        self.surface_config.width = size.x.max(1);
        self.surface_config.height = size.y.max(1);

        if !self.locked_resolution {
            self.resolution = size;
        }
    }

    fn set_accepting_text_input(&mut self, accept: bool) {
        let text_input = self.window.subsystem().text_input();

        if accept {
            text_input.start();
        } else {
            text_input.stop();
        }
    }
}

impl GameWindow for Sdl2GameWindow {
    fn position(&self) -> IVec2 {
        self.position
    }

    fn set_position(&mut self, position: IVec2) {
        use sdl2::video::WindowPos;
        self.window.set_position(
            WindowPos::Positioned(position.x),
            WindowPos::Positioned(position.y),
        );
        self.position = position;
    }

    fn fullscreen(&self) -> bool {
        use sdl2::video::FullscreenType;

        !matches!(self.window.fullscreen_state(), FullscreenType::Off)
    }

    fn set_fullscreen(&mut self, fullscreen: bool) {
        use sdl2::video::FullscreenType;

        let mode = if fullscreen {
            FullscreenType::Desktop
        } else {
            FullscreenType::Off
        };

        let _ = self.window.set_fullscreen(mode);
    }

    fn request_size(&mut self, size: UVec2) {
        let _ = self.window.set_size(size.x, size.y);
    }

    fn size(&self) -> UVec2 {
        self.size
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

    fn set_title(&mut self, title: &str) {
        let _ = self.window.set_title(title);
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
}

use framework_core::raw_window_handle::{
    DisplayHandle, HandleError, HasDisplayHandle, HasWindowHandle, WindowHandle,
};

impl HasWindowHandle for Sdl2GameWindow {
    fn window_handle(&self) -> Result<WindowHandle, HandleError> {
        self.window.window_handle()
    }
}

impl HasDisplayHandle for Sdl2GameWindow {
    fn display_handle(&self) -> Result<DisplayHandle, HandleError> {
        self.window.display_handle()
    }
}
