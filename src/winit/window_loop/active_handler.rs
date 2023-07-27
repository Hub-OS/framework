use crate::prelude::*;
use winit::event::Event as WinitEvent;
use winit::event::StartCause as WinitEventStartCause;
use winit::event_loop::ControlFlow;
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
        winit_event: WinitEvent<'_, ()>,
        control_flow: &mut ControlFlow,
    ) -> Option<Box<dyn super::WinitEventHandler>> {
        if self.game_runtime.is_quitting() {
            control_flow.set_exit();
            return None;
        }

        match winit_event {
            WinitEvent::NewEvents(WinitEventStartCause::Init)
            | WinitEvent::NewEvents(
                WinitEventStartCause::Poll | WinitEventStartCause::ResumeTimeReached { .. },
            ) => {
                self.controller_event_pump.pump(&mut self.game_runtime);
                self.game_runtime.tick();

                if self.game_runtime.game_io().is_suspended() {
                    control_flow.set_wait();
                } else {
                    control_flow.set_wait_until(self.game_runtime.target_sleep_instant());
                }
            }
            WinitEvent::Suspended => {
                self.game_runtime.set_suspended(true);
            }
            WinitEvent::Resumed => {
                control_flow.set_poll();
                self.game_runtime.set_suspended(false);
            }
            _ => {}
        }

        let window = self.game_runtime.game_io().window();

        if let Some(event) = translate_winit_event(window, self.window_id, winit_event) {
            self.game_runtime.push_event(event)
        }

        None
    }
}
