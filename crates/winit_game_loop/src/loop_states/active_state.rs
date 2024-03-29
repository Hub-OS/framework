use crate::event_translation::translate_winit_event;
use crate::{ControllerEventPump, WinitGameWindow};
use framework_core::runtime::*;
use winit::event::{Event as WinitEvent, StartCause as WinitEventStartCause};
use winit::event_loop::{ControlFlow, EventLoopWindowTarget};
use winit::window::WindowId;

use super::StartingStateParams;

pub struct ActiveState {
    window_id: WindowId,
    game_runtime: GameRuntimeCore,
    controller_event_pump: ControllerEventPump,
    handled_suspended: bool,
}

impl ActiveState {
    pub async fn new(params: StartingStateParams) -> anyhow::Result<Self> {
        let window_id = params.winit_window.id();
        let window =
            WinitGameWindow::from_window_and_config(params.winit_window, params.window_config)
                .await?;

        let mut game_runtime =
            GameRuntimeCore::new(Box::new(window), params.runtime_params).await?;

        let controller_event_pump = ControllerEventPump::new(&mut game_runtime)?;

        Ok(Self {
            window_id,
            game_runtime,
            controller_event_pump,
            handled_suspended: true,
        })
    }
}

impl super::LoopState for ActiveState {
    fn handle_event(
        &mut self,
        winit_event: WinitEvent<()>,
        event_loop_target: &EventLoopWindowTarget<()>,
    ) -> Option<Box<dyn super::LoopState>> {
        if self.game_runtime.quitting() {
            event_loop_target.exit();
            return None;
        }

        match winit_event {
            WinitEvent::NewEvents(
                WinitEventStartCause::Init
                | WinitEventStartCause::Poll
                | WinitEventStartCause::ResumeTimeReached { .. },
            ) => {
                self.controller_event_pump.pump(&mut self.game_runtime);
                self.game_runtime.tick();
                self.handled_suspended = true;
            }
            WinitEvent::Suspended => {
                self.game_runtime.set_suspended(true);
                self.handled_suspended = false;
            }
            WinitEvent::Resumed => {
                self.game_runtime.set_suspended(false);
            }
            WinitEvent::AboutToWait => {
                if self.game_runtime.game_io().suspended() && self.handled_suspended {
                    event_loop_target.set_control_flow(ControlFlow::Wait);
                } else {
                    event_loop_target.set_control_flow(ControlFlow::WaitUntil(
                        self.game_runtime.target_wake_instant(),
                    ));
                }
            }
            _ => {}
        }

        let window = self.game_runtime.game_io().window();

        for events in translate_winit_event(window, self.window_id, winit_event) {
            self.game_runtime.push_event(events);
        }

        None
    }
}
