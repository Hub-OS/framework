use crate::prelude::*;

pub(crate) struct WindowLoop {
    window: Window,
    event_pump: sdl2::EventPump,
    game_controller_subsystem: sdl2::GameControllerSubsystem,
}

impl WindowLoop {
    pub(super) fn new(
        window: Window,
        event_pump: sdl2::EventPump,
        game_controller_subsystem: sdl2::GameControllerSubsystem,
    ) -> Self {
        Self {
            window,
            event_pump,
            game_controller_subsystem,
        }
    }

    pub(crate) async fn run(mut self, loop_params: WindowLoopParams) -> anyhow::Result<()> {
        let window_id = self.window.id();
        let mut game_runtime = GameRuntime::new(self.window, loop_params).await?;

        while !game_runtime.is_quitting() {
            for sdl_event in self.event_pump.poll_iter() {
                if let sdl2::event::Event::JoyDeviceAdded { which, .. } = sdl_event {
                    if let Ok(controller) = self.game_controller_subsystem.open(which) {
                        game_runtime.push_event(
                            InputEvent::ControllerConnected {
                                controller_id: which as usize,
                                rumble_pack: RumblePack::new(controller),
                            }
                            .into(),
                        );
                    }
                }

                let window = game_runtime.game_io().window();

                if let Some(event) = translate_sdl_event(window, window_id, sdl_event) {
                    // reducing differences with winit
                    let text = match event {
                        WindowEvent::InputEvent(InputEvent::KeyDown(Key::Backspace)) => {
                            Some("\u{8}")
                        }
                        WindowEvent::InputEvent(InputEvent::KeyDown(Key::Delete)) => Some("\u{7f}"),
                        _ => None,
                    };

                    if let Some(text) = text {
                        game_runtime.push_event(WindowEvent::InputEvent(InputEvent::Text(
                            text.to_string(),
                        )));
                    }

                    game_runtime.push_event(event);
                }
            }

            game_runtime.tick();
            game_runtime.sleep().await;
        }

        Ok(())
    }
}
