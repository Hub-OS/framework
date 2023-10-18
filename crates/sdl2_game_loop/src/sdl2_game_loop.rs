use crate::{translate_sdl_event, Sdl2GameWindow, Sdl2RumblePack};
use framework_core::runtime::{
    GameRuntimeCore, GameRuntimeCoreParams, GameWindowConfig, GameWindowEvent, GameWindowLoop,
    InputEvent,
};
use input::*;
use std::future::Future;

pub struct Sdl2GameLoop {
    window: Sdl2GameWindow,
    event_pump: sdl2::EventPump,
    game_controller_subsystem: sdl2::GameControllerSubsystem,
}

impl GameWindowLoop for Sdl2GameLoop {
    type PlatformApp = crate::Sdl2PlatformApp;

    fn build(window_config: GameWindowConfig<Self::PlatformApp>) -> anyhow::Result<Sdl2GameLoop> {
        let sdl_context = sdl2::init().map_err(|e| anyhow::anyhow!(e))?;
        let event_pump = sdl_context.event_pump().map_err(|e| anyhow::anyhow!(e))?;

        let video_subsystem = sdl_context.video().map_err(|e| anyhow::anyhow!(e))?;
        let mut sdl_window_builder = video_subsystem.window(
            &window_config.title,
            window_config.size.0,
            window_config.size.1,
        );

        sdl_window_builder.position_centered();

        if window_config.resizable {
            sdl_window_builder.resizable();
        }

        if window_config.fullscreen {
            sdl_window_builder.fullscreen();
        }

        if window_config.borderless {
            sdl_window_builder.borderless();
        }

        let sdl_window = sdl_window_builder.build()?;

        let window = Sdl2GameWindow::from_window_and_config(sdl_window, window_config);

        let game_controller_subsystem = sdl_context
            .game_controller()
            .map_err(|e| anyhow::anyhow!(e))?;

        let window_loop = Self {
            window,
            event_pump,
            game_controller_subsystem,
        };

        Ok(window_loop)
    }

    fn run(
        mut self,
        loop_params: GameRuntimeCoreParams,
    ) -> Box<dyn Future<Output = anyhow::Result<()>>> {
        Box::new(async move {
            let window_id = self.window.id();
            let mut game_runtime = GameRuntimeCore::new(self.window, loop_params).await?;

            let game_io = game_runtime.game_io_mut();
            game_io.input_mut().set_default_controller_deadzone(0.05);

            while !game_runtime.quitting() {
                for sdl_event in self.event_pump.poll_iter() {
                    if let sdl2::event::Event::JoyDeviceAdded { which, .. } = sdl_event {
                        if let Ok(controller) = self.game_controller_subsystem.open(which) {
                            game_runtime.push_event(
                                InputEvent::ControllerConnected {
                                    controller_id: which as usize,
                                    rumble_pack: Box::from(Sdl2RumblePack::new(controller)),
                                }
                                .into(),
                            );
                        }
                    }

                    let window = game_runtime.game_io().window();

                    if let Some(event) = translate_sdl_event(window, window_id, sdl_event) {
                        // reducing differences with winit
                        let text = match event {
                            GameWindowEvent::InputEvent(InputEvent::KeyDown(Key::Backspace)) => {
                                Some("\u{8}")
                            }
                            GameWindowEvent::InputEvent(InputEvent::KeyDown(Key::Delete)) => {
                                Some("\u{7f}")
                            }
                            _ => None,
                        };

                        if let Some(text) = text {
                            game_runtime.push_event(GameWindowEvent::InputEvent(InputEvent::Text(
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
        })
    }
}
