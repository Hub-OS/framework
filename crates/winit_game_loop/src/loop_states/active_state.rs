use super::LoopState;
use crate::event_translation::translate_winit_event;
use crate::{ControllerEventPump, WinitGameWindow};
use framework_core::runtime::*;
use winit::application::ApplicationHandler;
use winit::event::StartCause as WinitEventStartCause;
use winit::event_loop::{ActiveEventLoop, ControlFlow};
use winit::window::WindowId;

pub struct ActiveStateParams {
    pub winit_window: winit::window::Window,
    pub window_config: GameWindowConfig<crate::WinitPlatformApp>,
    pub runtime_params: GameRuntimeCoreParams,
}

pub struct ActiveState {
    window_id: WindowId,
    game_runtime: GameRuntimeCore,
    controller_event_pump: ControllerEventPump,
    handled_suspended: bool,
    next_state: Option<Box<dyn LoopState>>,
}

impl ActiveState {
    pub async fn new(params: ActiveStateParams) -> anyhow::Result<Self> {
        let window_id = params.winit_window.id();
        let window =
            WinitGameWindow::from_window_and_config(params.winit_window, params.window_config)
                .await?;

        let mut game_runtime = GameRuntimeCore::new(Box::new(window), params.runtime_params)?;

        let controller_event_pump = ControllerEventPump::new(&mut game_runtime)?;

        Ok(Self {
            window_id,
            game_runtime,
            controller_event_pump,
            handled_suspended: true,
            next_state: None,
        })
    }
}

impl LoopState for ActiveState {
    fn next_state(&mut self) -> Option<Box<dyn LoopState>> {
        self.next_state.take()
    }
}

impl ApplicationHandler for ActiveState {
    fn new_events(&mut self, _: &ActiveEventLoop, cause: WinitEventStartCause) {
        if matches!(
            cause,
            WinitEventStartCause::Init
                | WinitEventStartCause::Poll
                | WinitEventStartCause::ResumeTimeReached { .. },
        ) {
            self.controller_event_pump.pump(&mut self.game_runtime);
            self.game_runtime.tick();
            self.handled_suspended = true;
        }
    }

    fn resumed(&mut self, _: &ActiveEventLoop) {
        self.game_runtime.set_suspended(false);
        self.game_runtime.push_event(GameWindowEvent::Resumed);
    }

    fn suspended(&mut self, _: &ActiveEventLoop) {
        self.game_runtime.set_suspended(true);
        self.handled_suspended = false;
    }

    fn window_event(
        &mut self,
        _: &ActiveEventLoop,
        window_id: WindowId,
        event: winit::event::WindowEvent,
    ) {
        if self.window_id == window_id {
            let window = self.game_runtime.game_io().window();

            for event in translate_winit_event(window, event) {
                self.game_runtime.push_event(event);
            }
        }
    }

    fn about_to_wait(&mut self, event_loop: &ActiveEventLoop) {
        if self.game_runtime.game_io().suspended() && self.handled_suspended {
            event_loop.set_control_flow(ControlFlow::Wait);
        } else {
            event_loop.set_control_flow(ControlFlow::WaitUntil(
                self.game_runtime.target_wake_instant(),
            ));
        }

        if self.game_runtime.quitting() {
            event_loop.exit();
        }
    }
}
