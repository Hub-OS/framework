use super::*;
use crate::cfg_android;
use cfg_macros::cfg_web;
use framework_core::runtime::GameRuntimeCoreParams;
use framework_core::runtime::GameWindowConfig;
use logging::log;
use winit::application::ApplicationHandler;
use winit::dpi::PhysicalSize;
use winit::event_loop::ActiveEventLoop;
use winit::event_loop::ControlFlow;
use winit::window::WindowLevel;

pub struct StartingStateParams {
    pub window_config: GameWindowConfig<crate::WinitPlatformApp>,
    pub runtime_params: GameRuntimeCoreParams,
}

pub struct StartingState {
    async_executor: async_executor::LocalExecutor<'static>,
    params: Option<StartingStateParams>,
    task: Option<async_executor::Task<anyhow::Result<ActiveState>>>,
    next_state: Option<Box<dyn LoopState>>,
}

impl StartingState {
    pub fn new(params: StartingStateParams) -> Self {
        #[allow(unused_mut)]
        let mut starting_state = Self {
            async_executor: async_executor::LocalExecutor::new(),
            params: Some(params),
            task: None,
            next_state: None,
        };

        starting_state
    }
}

impl LoopState for StartingState {
    fn next_state(&mut self) -> Option<Box<dyn LoopState>> {
        self.next_state.take()
    }
}

impl ApplicationHandler for StartingState {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let params = self.params.take().unwrap();
        let window_config = params.window_config;

        let mut window_attributes = winit::window::Window::default_attributes()
            .with_title(&window_config.title)
            .with_inner_size(PhysicalSize::new(
                window_config.size.x,
                window_config.size.y,
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
            window_attributes =
                window_attributes.with_fullscreen(Some(Fullscreen::Borderless(None)));

            cfg_android!({
                if let Some(app) = &window_config.platform_app {
                    android::util::hide_system_bars(app);
                }
            });
        }

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

        let active_state_params = ActiveStateParams {
            winit_window: event_loop.create_window(window_attributes).unwrap(),
            window_config,
            runtime_params: params.runtime_params,
        };

        self.task = Some(
            self.async_executor
                .spawn(ActiveState::new(active_state_params)),
        );
        event_loop.set_control_flow(ControlFlow::Poll);
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _: winit::window::WindowId,
        event: winit::event::WindowEvent,
    ) {
        if event == winit::event::WindowEvent::CloseRequested {
            event_loop.exit();
        }
    }

    fn about_to_wait(&mut self, event_loop: &ActiveEventLoop) {
        while self.async_executor.try_tick() {}

        let task_ref = self.task.as_ref();
        let task_completed = task_ref.map(|task| task.is_finished()).unwrap_or_default();

        if task_completed {
            let task = self.task.take().unwrap();
            let task_value = framework_core::async_task::block_on(task.cancel()).unwrap();

            match task_value {
                Ok(new_state) => {
                    self.next_state = Some(Box::new(new_state));
                }
                Err(e) => {
                    log::error!("{}", e);
                    event_loop.exit();
                }
            }
        }
    }
}
