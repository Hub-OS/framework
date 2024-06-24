use winit::application::ApplicationHandler;
use winit::event::{StartCause, WindowEvent};
use winit::event_loop::ActiveEventLoop;
use winit::window::WindowId;

pub trait LoopState: ApplicationHandler {
    fn next_state(&mut self) -> Option<Box<dyn LoopState>>;
}

pub struct RootLoopState {
    state: Box<dyn LoopState>,
}

impl RootLoopState {
    pub fn new(state: impl LoopState + 'static) -> Self {
        Self {
            state: Box::new(state),
        }
    }
}

impl ApplicationHandler for RootLoopState {
    fn new_events(&mut self, event_loop: &ActiveEventLoop, cause: StartCause) {
        self.state.new_events(event_loop, cause);
    }

    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        self.state.resumed(event_loop)
    }

    fn suspended(&mut self, event_loop: &ActiveEventLoop) {
        self.state.suspended(event_loop)
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        window_id: WindowId,
        event: WindowEvent,
    ) {
        self.state.window_event(event_loop, window_id, event)
    }

    fn about_to_wait(&mut self, event_loop: &ActiveEventLoop) {
        self.state.about_to_wait(event_loop);

        if let Some(new_state) = self.state.next_state() {
            self.state = new_state;

            self.state.new_events(event_loop, StartCause::Init);
        }
    }
}
