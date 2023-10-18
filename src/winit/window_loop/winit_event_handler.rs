use winit::event::Event as WinitEvent;
use winit::event_loop::EventLoopWindowTarget;

pub(super) trait WinitEventHandler {
    fn handle_event(
        &mut self,
        winit_event: WinitEvent<()>,
        event_loop_target: &EventLoopWindowTarget<()>,
    ) -> Option<Box<dyn WinitEventHandler>>;
}
