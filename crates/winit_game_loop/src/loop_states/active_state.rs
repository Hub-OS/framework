use crate::event_translation::translate_winit_event;
use crate::{ControllerEventPump, WinitGameWindow};
use framework_core::runtime::*;
use winit::event::{Event as WinitEvent, StartCause as WinitEventStartCause};
use winit::event_loop::{ControlFlow, EventLoopWindowTarget};
use winit::window::WindowId;

pub struct ActiveState {
    window_id: WindowId,
    game_runtime: GameRuntimeCore,
    controller_event_pump: ControllerEventPump,
}

impl ActiveState {
    pub async fn new(
        window: WinitGameWindow,
        window_id: WindowId,
        loop_params: GameRuntimeCoreParams,
    ) -> anyhow::Result<Self> {
        let mut game_runtime = GameRuntimeCore::new(Box::new(window), loop_params).await?;
        let controller_event_pump = ControllerEventPump::new(&mut game_runtime)?;

        Ok(Self {
            window_id,
            game_runtime,
            controller_event_pump,
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
            }
            WinitEvent::Suspended => {
                self.game_runtime.set_suspended(true);
            }
            WinitEvent::Resumed => {
                self.game_runtime.set_suspended(false);
            }
            WinitEvent::AboutToWait => {
                if self.game_runtime.game_io().suspended() {
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
