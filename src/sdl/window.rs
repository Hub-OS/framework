use crate::prelude::*;

pub struct Window {
    window: sdl2::video::Window,
    position: IVec2,
    size: UVec2,
    resolution: UVec2,
    locked_resolution: bool,
}

impl Window {
    pub fn position(&self) -> IVec2 {
        self.position
    }

    pub fn set_position(&mut self, position: IVec2) {
        use sdl2::video::WindowPos;
        self.window.set_position(
            WindowPos::Positioned(position.x),
            WindowPos::Positioned(position.y),
        );
        self.position = position;
    }

    pub fn fullscreen(&self) -> bool {
        use sdl2::video::FullscreenType;

        !matches!(self.window.fullscreen_state(), FullscreenType::Off)
    }

    pub fn set_fullscreen(&mut self, fullscreen: bool) {
        use sdl2::video::FullscreenType;

        let mode = if fullscreen {
            FullscreenType::Desktop
        } else {
            FullscreenType::Off
        };

        let _ = self.window.set_fullscreen(mode);
    }

    pub fn size(&self) -> UVec2 {
        self.size
    }

    pub fn has_locked_resolution(&self) -> bool {
        self.locked_resolution
    }

    pub fn lock_resolution(&mut self, resolution: UVec2) {
        self.resolution = resolution;
        self.locked_resolution = true;
    }

    pub fn unlock_resolution(&mut self) {
        self.resolution = self.size;
        self.locked_resolution = false;
    }

    pub fn resolution(&self) -> UVec2 {
        self.resolution
    }

    pub fn set_title(&mut self, title: &str) {
        let _ = self.window.set_title(title);
    }

    pub(crate) fn build(window_config: WindowConfig) -> anyhow::Result<WindowLoop> {
        let sdl_context = sdl2::init().map_err(|e| anyhow::anyhow!(e))?;
        let event_pump = sdl_context.event_pump().map_err(|e| anyhow::anyhow!(e))?;

        let video_subsystem = sdl_context.video().map_err(|e| anyhow::anyhow!(e))?;
        let mut sdl_window_builder = video_subsystem.window(
            &window_config.title,
            window_config.size.0,
            window_config.size.1,
        );

        sdl_window_builder.position_centered();

        if window_config.resizable {
            sdl_window_builder.resizable();
        }

        if window_config.fullscreen {
            sdl_window_builder.fullscreen();
        }

        if window_config.borderless {
            sdl_window_builder.borderless();
        }

        let sdl_window = sdl_window_builder.build()?;

        let size = sdl_window.size();

        let window = Window {
            size: sdl_window.size().into(),
            position: sdl_window.position().into(),
            window: sdl_window,
            locked_resolution: window_config.resolution.is_some(),
            resolution: window_config.resolution.unwrap_or(size).into(),
        };

        let game_controller_subsystem = sdl_context
            .game_controller()
            .map_err(|e| anyhow::anyhow!(e))?;

        let window_loop = WindowLoop::new(window, event_pump, game_controller_subsystem);

        Ok(window_loop)
    }

    pub(crate) fn id(&self) -> u32 {
        self.window.id()
    }

    pub(crate) fn moved(&mut self, position: IVec2) {
        self.position = position;
    }

    pub(crate) fn resized(&mut self, size: UVec2) {
        self.size = size;

        if !self.locked_resolution {
            self.resolution = size;
        }
    }

    pub(crate) fn set_text_input(&mut self, accept: bool) {
        let text_input = self.window.subsystem().text_input();

        if accept {
            text_input.start();
        } else {
            text_input.stop();
        }
    }
}

use raw_window_handle::{
    HasRawDisplayHandle, HasRawWindowHandle, RawDisplayHandle, RawWindowHandle,
};

unsafe impl HasRawWindowHandle for Window {
    fn raw_window_handle(&self) -> RawWindowHandle {
        self.window.raw_window_handle()
    }
}

unsafe impl HasRawDisplayHandle for Window {
    fn raw_display_handle(&self) -> RawDisplayHandle {
        self.window.raw_display_handle()
    }
}
