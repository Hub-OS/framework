use super::*;
use crate::loop_states::{LoopState, StartingState};
use crate::WinitGameWindow;
use cfg_macros::*;
use framework_core::runtime::{GameRuntimeCoreParams, GameWindowConfig, GameWindowLoop};
use std::future::Future;
use winit::dpi::PhysicalSize;
use winit::event::{Event as WinitEvent, StartCause as WinitEventStartCause};
use winit::event_loop::EventLoop;
use winit::window::{WindowBuilder, WindowLevel};

pub struct WinitGameLoop {
    window: WinitGameWindow,
    event_loop: EventLoop<()>,
}

impl GameWindowLoop for WinitGameLoop {
    type PlatformApp = crate::WinitPlatformApp;

    fn build(window_config: GameWindowConfig<Self::PlatformApp>) -> anyhow::Result<Self> {
        let event_loop = create_winit_event_loop(window_config.platform_app.clone())?;

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

            cfg_android!({
                use crate::android;

                if let Some(app) = &window_config.platform_app {
                    android::hide_system_bars(app);
                }
            });
        }

        let winit_window = winit_window_builder.build(&event_loop)?;

        cfg_web!({
            use wasm_forward::web_sys;
            use winit::platform::web::WindowExtWebSys;

            let document = web_sys::window().unwrap().document().unwrap();

            let canvas = winit_window.canvas().unwrap();

            canvas.style().set_property("outline", "0").unwrap();

            document
                .body()
                .unwrap()
                .append_child(&canvas)
                .expect("Couldn't append canvas to document body.");
        });

        let window = WinitGameWindow::from_window_and_config(winit_window, window_config);

        let window_loop = Self { window, event_loop };

        Ok(window_loop)
    }

    fn run(self, params: GameRuntimeCoreParams) -> Box<dyn Future<Output = anyhow::Result<()>>> {
        Box::new(async {
            let mut state: Box<dyn LoopState> = Box::new(StartingState::new(self.window, params));

            self.event_loop.run(move |winit_event, event_loop_target| {
                if let Some(new_state) = state.handle_event(winit_event, event_loop_target) {
                    state = new_state;

                    state.handle_event(
                        WinitEvent::NewEvents(WinitEventStartCause::Init),
                        event_loop_target,
                    );
                }
            })?;

            Ok(())
        })
    }
}

#[allow(unused_variables)]
fn create_winit_event_loop(
    platform_app: Option<WinitPlatformApp>,
) -> anyhow::Result<EventLoop<()>> {
    cfg_android! {
        if let Some(app) = platform_app {
            use winit::platform::android::EventLoopBuilderExtAndroid;
            use winit::event_loop::EventLoopBuilder;

            return Ok(EventLoopBuilder::new().with_android_app(app).build()?)
        }
    };

    Ok(EventLoop::new()?)
}
