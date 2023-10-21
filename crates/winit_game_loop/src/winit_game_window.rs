use crate::WinitPlatformApp;
use cfg_macros::*;
use framework_core::common::GameWindow;
use framework_core::graphics::Color;
use framework_core::graphics::{wgpu, GraphicsContext, HasGraphicsContext, RenderTarget};
use framework_core::runtime::{GameWindowConfig, GameWindowLifecycle};
use math::*;
use winit::window::Window;

pub struct WinitGameWindow {
    window: winit::window::Window,
    graphics: GraphicsContext,
    surface: wgpu::Surface,
    surface_config: wgpu::SurfaceConfiguration,
    surface_texture: Option<wgpu::SurfaceTexture>,
    position: IVec2,
    size: UVec2,
    resolution: UVec2,
    locked_resolution: bool,
    clear_color: Option<Color>,
    #[allow(dead_code)]
    platform_app: Option<WinitPlatformApp>,
}

impl WinitGameWindow {
    pub(crate) async fn from_window_and_config(
        window: Window,
        window_config: GameWindowConfig<WinitPlatformApp>,
    ) -> anyhow::Result<Self> {
        let position = window.outer_position().unwrap_or_default();

        let wgpu_instance = wgpu::Instance::default();
        let surface = unsafe { wgpu_instance.create_surface(&window).unwrap() };
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
            window,
            graphics,
            surface,
            surface_config,
            surface_texture: None,
            position: IVec2::new(position.x, position.y),
            size: window_config.size,
            resolution: window_config.resolution.unwrap_or(window_config.size),
            locked_resolution: window_config.resolution.is_some(),
            clear_color: Some(Color::TRANSPARENT),
            platform_app: window_config.platform_app.clone(),
        })
    }
}

impl HasGraphicsContext for WinitGameWindow {
    fn graphics(&self) -> &GraphicsContext {
        &self.graphics
    }
}

impl GameWindowLifecycle for WinitGameWindow {
    fn rebuild_surface(&mut self) {
        let graphics = self.graphics();
        let instance = graphics.wgpu_instance();
        let device = graphics.device();

        if let Ok(surface) = unsafe { instance.create_surface(&self.window) } {
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

        self.surface_config.width = size.x;
        self.surface_config.height = size.y;

        let device = self.graphics().device();
        self.surface.configure(device, &self.surface_config);
    }

    fn set_accepting_text_input(&mut self, accept: bool) {
        cfg_android!({
            use crate::android;

            if accept {
                if let Some(app) = &self.platform_app {
                    android::show_ime(app);
                }
            }
        });

        cfg_desktop_and_web!({
            self.window.set_ime_allowed(accept);
        })
    }
}

impl GameWindow for WinitGameWindow {
    fn position(&self) -> IVec2 {
        self.position
    }

    fn set_position(&mut self, position: IVec2) {
        use winit::dpi::LogicalPosition;
        self.window
            .set_outer_position(LogicalPosition::new(position.x, position.y));
        self.position = position;
    }

    fn fullscreen(&self) -> bool {
        self.window.fullscreen().is_some()
    }

    fn set_fullscreen(&mut self, fullscreen: bool) {
        use winit::window::Fullscreen;

        let mode = if fullscreen {
            Some(Fullscreen::Borderless(None))
        } else {
            None
        };

        self.window.set_fullscreen(mode);

        cfg_android!({
            use crate::android;

            if let Some(app) = &self.platform_app {
                if fullscreen {
                    android::hide_system_bars(app)
                } else {
                    android::show_system_bars(app)
                }
            }
        });
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

    fn set_title(&mut self, title: &str) {
        self.window.set_title(title);
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
    HasRawDisplayHandle, HasRawWindowHandle, RawDisplayHandle, RawWindowHandle,
};

unsafe impl HasRawWindowHandle for WinitGameWindow {
    fn raw_window_handle(&self) -> RawWindowHandle {
        self.window.raw_window_handle()
    }
}

unsafe impl HasRawDisplayHandle for WinitGameWindow {
    fn raw_display_handle(&self) -> RawDisplayHandle {
        self.window.raw_display_handle()
    }
}
