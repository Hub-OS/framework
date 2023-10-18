use super::*;
use crate::prelude::*;
use winit::event::Event as WinitEvent;
use winit::event::StartCause as WinitEventStartCause;
use winit::event_loop::EventLoop;

pub(crate) struct WindowLoop {
    window: Window,
    event_loop: EventLoop<()>,
}

impl WindowLoop {
    pub(crate) fn new(window: Window, event_loop: EventLoop<()>) -> Self {
        Self { window, event_loop }
    }

    pub(crate) async fn run(self, params: WindowLoopParams) -> anyhow::Result<()> {
        let mut event_handler: Box<dyn WinitEventHandler> =
            Box::new(StartingHandler::new(self.window, params));

        self.event_loop.run(move |winit_event, event_loop_target| {
            if let Some(new_handler) = event_handler.handle_event(winit_event, event_loop_target) {
                event_handler = new_handler;

                event_handler.handle_event(
                    WinitEvent::NewEvents(WinitEventStartCause::Init),
                    event_loop_target,
                );
            }
        })?;

        Ok(())
    }
}
