use winit::event::Event as WinitEvent;
use winit::event_loop::EventLoopWindowTarget;

pub trait LoopState {
    fn handle_event(
        &mut self,
        winit_event: WinitEvent<()>,
        event_loop_target: &EventLoopWindowTarget<()>,
    ) -> Option<Box<dyn LoopState>>;
}
