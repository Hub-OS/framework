use crate::Sdl2PlatformApp;
use framework_core::common::GameWindow;
use framework_core::runtime::GameWindowConfig;
use math::*;

pub struct Sdl2GameWindow {
    window: sdl2::video::Window,
    position: IVec2,
    size: UVec2,
    resolution: UVec2,
    locked_resolution: bool,
}

impl Sdl2GameWindow {
    pub(crate) fn from_window_and_config(
        window: sdl2::video::Window,
        window_config: GameWindowConfig<Sdl2PlatformApp>,
    ) -> Self {
        let size = window.size();

        Self {
            size: window.size().into(),
            position: window.position().into(),
            window,
            locked_resolution: window_config.resolution.is_some(),
            resolution: window_config.resolution.unwrap_or(size).into(),
        }
    }

    pub(crate) fn id(&self) -> u32 {
        self.window.id()
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
        let _ = self.window.set_title(title);
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
        let text_input = self.window.subsystem().text_input();

        if accept {
            text_input.start();
        } else {
            text_input.stop();
        }
    }
}

use framework_core::raw_window_handle::{
    HasRawDisplayHandle, HasRawWindowHandle, RawDisplayHandle, RawWindowHandle,
};

unsafe impl HasRawWindowHandle for Sdl2GameWindow {
    fn raw_window_handle(&self) -> RawWindowHandle {
        self.window.raw_window_handle()
    }
}

unsafe impl HasRawDisplayHandle for Sdl2GameWindow {
    fn raw_display_handle(&self) -> RawDisplayHandle {
        self.window.raw_display_handle()
    }
}
