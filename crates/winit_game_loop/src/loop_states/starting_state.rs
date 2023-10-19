use super::*;
use crate::cfg_android;
use crate::WinitGameWindow;
use framework_core::runtime::GameRuntimeCoreParams;
use logging::error;
use winit::event::Event as WinitEvent;
use winit::event_loop::ControlFlow;
use winit::event_loop::EventLoopWindowTarget;

struct StartingStateInitParams {
    window: WinitGameWindow,
    runtime_params: GameRuntimeCoreParams,
}

pub struct StartingState {
    async_executor: async_executor::LocalExecutor<'static>,
    starting_state_init_params: Option<StartingStateInitParams>,
    task: Option<async_executor::Task<anyhow::Result<ActiveState>>>,
}

impl StartingState {
    pub fn new(window: WinitGameWindow, runtime_params: GameRuntimeCoreParams) -> Self {
        #[allow(unused_mut)]
        let mut starting_state = Self {
            async_executor: async_executor::LocalExecutor::new(),
            starting_state_init_params: Some(StartingStateInitParams {
                window,
                runtime_params,
            }),
            task: None,
        };

        crate::cfg_desktop_and_web!({
            starting_state.start_active_state_task();
        });

        starting_state
    }
}

impl StartingState {
    fn start_active_state_task(&mut self) {
        if let Some(params) = self.starting_state_init_params.take() {
            let window_id = params.window.id();

            self.task = Some(self.async_executor.spawn(ActiveState::new(
                params.window,
                window_id,
                params.runtime_params,
            )));
        }
    }
}

impl LoopState for StartingState {
    fn handle_event(
        &mut self,
        winit_event: WinitEvent<()>,
        event_loop_target: &EventLoopWindowTarget<()>,
    ) -> Option<Box<dyn LoopState>> {
        match winit_event {
            WinitEvent::Resumed => {
                cfg_android!(self.start_active_state_task());
                event_loop_target.set_control_flow(ControlFlow::Poll);
            }
            WinitEvent::WindowEvent {
                event: winit::event::WindowEvent::CloseRequested,
                ..
            } => {
                event_loop_target.exit();
            }
            WinitEvent::AboutToWait => {
                while self.async_executor.try_tick() {}

                let task_ref = self.task.as_ref();
                let task_completed = task_ref.map(|task| task.is_finished()).unwrap_or_default();

                if task_completed {
                    let task = self.task.take().unwrap();
                    let task_value = framework_core::async_task::block_on(task.cancel()).unwrap();

                    match task_value {
                        Ok(new_state) => {
                            return Some(Box::new(new_state));
                        }
                        Err(e) => {
                            error!("{}", e);
                            event_loop_target.exit();
                        }
                    }
                }
            }
            _ => {}
        }

        None
    }
}
