use super::WindowLoop;
use crate::{cfg_android, prelude::*};
use winit::dpi::PhysicalSize;
use winit::event_loop::EventLoop;
use winit::window::{WindowBuilder, WindowLevel};

pub struct Window {
    window: winit::window::Window,
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
        use winit::dpi::LogicalPosition;
        self.window
            .set_outer_position(LogicalPosition::new(position.x, position.y));
        self.position = position;
    }

    pub fn fullscreen(&self) -> bool {
        self.window.fullscreen().is_some()
    }

    pub fn set_fullscreen(&mut self, fullscreen: bool) {
        use winit::window::Fullscreen;

        let mode = if fullscreen {
            Some(Fullscreen::Borderless(None))
        } else {
            None
        };

        self.window.set_fullscreen(mode);
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
        self.window.set_title(title);
    }

    pub fn normalize_vec2(&self, mut position: Vec2) -> Vec2 {
        let window_size = self.size().as_vec2();
        let scale = window_size / self.resolution().as_vec2();

        position.x = position.x / window_size.x * 2.0 - 1.0;
        position.y = -(position.y / window_size.y * 2.0 - 1.0);

        if scale.x > scale.y {
            position.x *= scale.x / scale.y;
        } else {
            position.y *= scale.y / scale.x;
        }

        position
    }

    #[allow(unused_variables)]
    fn create_winit_event_loop(platform_app: Option<PlatformApp>) -> EventLoop<()> {
        cfg_android! {
            if let Some(app) = platform_app {
                use winit::platform::android::EventLoopBuilderExtAndroid;
                use winit::event_loop::EventLoopBuilder;

                return EventLoopBuilder::new().with_android_app(app).build()
            }
        };

        EventLoop::new()
    }

    pub(crate) fn build(window_config: WindowConfig) -> anyhow::Result<WindowLoop> {
        let event_loop = Self::create_winit_event_loop(window_config.platform_app);

        let mut winit_window_builder = WindowBuilder::new()
            .with_title(&window_config.title)
            .with_inner_size(PhysicalSize::new(
                window_config.size.0,
                window_config.size.1,
            ))
            .with_resizable(window_config.resizable)
            .with_decorations(!window_config.borderless)
            .with_transparent(window_config.transparent)
            .with_window_level(if window_config.always_on_top {
                WindowLevel::AlwaysOnTop
            } else {
                WindowLevel::Normal
            });

        if window_config.fullscreen {
            use winit::window::Fullscreen;
            winit_window_builder =
                winit_window_builder.with_fullscreen(Some(Fullscreen::Borderless(None)));
        }

        let winit_window = winit_window_builder.build(&event_loop)?;

        crate::cfg_web!({
            use wasm_bindgen::JsCast;
            use web_sys::{Element, HtmlCanvasElement};
            use winit::platform::web::WindowExtWebSys;

            let document = web_sys::window().unwrap().document().unwrap();

            let canvas = Element::from(winit_window.canvas())
                .dyn_into::<HtmlCanvasElement>()
                .unwrap();

            canvas.style().set_property("outline", "0").unwrap();

            document
                .body()
                .unwrap()
                .append_child(&canvas)
                .expect("Couldn't append canvas to document body.");
        });

        let position = winit_window.outer_position().unwrap_or_default();

        let window = Window {
            window: winit_window,
            position: IVec2::new(position.x, position.y),
            size: window_config.size.into(),
            resolution: window_config
                .resolution
                .unwrap_or(window_config.size)
                .into(),
            locked_resolution: window_config.resolution.is_some(),
        };

        let window_loop = WindowLoop::new(window, event_loop);

        Ok(window_loop)
    }

    pub(crate) fn id(&self) -> winit::window::WindowId {
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
        self.window.set_ime_allowed(accept);
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
