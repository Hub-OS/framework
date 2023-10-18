use crate::WinitPlatformApp;
use cfg_macros::*;
use framework_core::{common::GameWindow, runtime::GameWindowConfig};
use math::*;
use winit::window::Window;

pub struct WinitGameWindow {
    window: winit::window::Window,
    position: IVec2,
    size: UVec2,
    resolution: UVec2,
    locked_resolution: bool,
    #[allow(dead_code)]
    platform_app: Option<WinitPlatformApp>,
}

impl WinitGameWindow {
    pub(crate) fn from_window_and_config(
        window: Window,
        window_config: GameWindowConfig<WinitPlatformApp>,
    ) -> Self {
        let position = window.outer_position().unwrap_or_default();

        Self {
            window,
            position: IVec2::new(position.x, position.y),
            size: window_config.size.into(),
            resolution: window_config
                .resolution
                .unwrap_or(window_config.size)
                .into(),
            locked_resolution: window_config.resolution.is_some(),
            platform_app: window_config.platform_app.clone(),
        }
    }

    pub(crate) fn id(&self) -> winit::window::WindowId {
        self.window.id()
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

    fn set_moved(&mut self, position: IVec2) {
        self.position = position;
    }

    fn set_resized(&mut self, size: UVec2) {
        self.size = size;

        if !self.locked_resolution {
            self.resolution = size;
        }
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
