use crate::prelude::*;
use winit::event::Event as WinitEvent;
use winit::event::StartCause as WinitEventStartCause;
use winit::event_loop::ControlFlow;
use winit::event_loop::EventLoopWindowTarget;
use winit::window::WindowId;

pub(super) struct ActiveHandler {
    window_id: WindowId,
    game_runtime: GameRuntime,
    controller_event_pump: ControllerEventPump,
}

impl ActiveHandler {
    pub(super) async fn new(window: Window, loop_params: WindowLoopParams) -> anyhow::Result<Self> {
        let window_id = window.id();
        let mut game_runtime = GameRuntime::new(window, loop_params).await?;
        let controller_event_pump = ControllerEventPump::new(&mut game_runtime)?;

        Ok(Self {
            window_id,
            game_runtime,
            controller_event_pump,
        })
    }
}

impl super::WinitEventHandler for ActiveHandler {
    fn handle_event(
        &mut self,
        winit_event: WinitEvent<()>,
        event_loop_target: &EventLoopWindowTarget<()>,
    ) -> Option<Box<dyn super::WinitEventHandler>> {
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
