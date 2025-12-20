use crate::android_rumble_pack::AndroidRumblePack;
use crate::{android_game_window::AndroidGameWindow, event_translation::translate_input_event};
use android::activity::{MainEvent as AndroidMainEvent, PollEvent as AndroidPollEvent};
use android_activity::input::Axis as AndroidAxis;
use android_activity::InputStatus as AndroidInputStatus;
use framework_core::runtime::*;
use math::UVec2;
use std::future::Future;
use std::time::Instant;

pub struct AndroidGameLoop {}

impl GameWindowLoop for AndroidGameLoop {
    type PlatformApp = crate::AndroidPlatformApp;

    fn run(
        window_config: GameWindowConfig<Self::PlatformApp>,
        runtime_params: GameRuntimeCoreParams,
    ) -> Box<dyn Future<Output = anyhow::Result<()>>> {
        Box::new(run(window_config, runtime_params))
    }
}

async fn run(
    window_config: GameWindowConfig<crate::AndroidPlatformApp>,
    runtime_params: GameRuntimeCoreParams,
) -> anyhow::Result<()> {
    let app = window_config.platform_app.clone().unwrap();

    // wait until we're ready to create a window
    loop {
        let mut terminated = false;
        let mut ready = false;

        app.poll_events(None, |event| {
            if let AndroidPollEvent::Main(main_event) = event {
                match main_event {
                    AndroidMainEvent::InitWindow { .. } => ready = true,
                    AndroidMainEvent::Destroy => terminated = true,
                    _ => {}
                }
            }
        });

        if terminated {
            return Ok(());
        }

        if ready {
            break;
        }
    }

    // only AndroidAxis::X and AndroidAxis::Y are enabled by default
    app.enable_motion_axis(AndroidAxis::Ltrigger);
    app.enable_motion_axis(AndroidAxis::Rtrigger);
    // Dpad
    app.enable_motion_axis(AndroidAxis::HatX);
    app.enable_motion_axis(AndroidAxis::HatY);
    // Right thumbstick
    app.enable_motion_axis(AndroidAxis::Z);
    app.enable_motion_axis(AndroidAxis::Rz);

    // init the window and runtime
    let window = AndroidGameWindow::new(window_config).await?;
    let mut game_runtime = GameRuntimeCore::new(Box::new(window), runtime_params)?;

    let mut combining_accent = None;

    game_runtime.push_event(GameWindowEvent::InputEvent(
        InputEvent::ControllerConnected {
            controller_id: 0,
            rumble_pack: Box::new(AndroidRumblePack),
        },
    ));

    while !game_runtime.quitting() {
        let wake_instant = game_runtime.target_wake_instant();
        let mut timeout = wake_instant.saturating_duration_since(Instant::now());

        // tick check
        if timeout.is_zero() {
            game_runtime.tick();

            // update timeout
            let wake_instant = game_runtime.target_wake_instant();
            timeout = wake_instant.saturating_duration_since(Instant::now());
        }

        let mut terminated = false;

        app.poll_events(Some(timeout), |event| {
            if let AndroidPollEvent::Main(main_event) = event {
                match main_event {
                    AndroidMainEvent::InitWindow { .. } => {
                        game_runtime.push_event(GameWindowEvent::Created);
                    }
                    AndroidMainEvent::Resume { .. } => {
                        game_runtime.set_suspended(false);
                    }
                    AndroidMainEvent::Pause { .. } => {
                        game_runtime.set_suspended(true);
                    }
                    AndroidMainEvent::WindowResized { .. } => {
                        if let Some(window) = app.native_window() {
                            let size = UVec2::new(window.width() as _, window.height() as _);
                            game_runtime.push_event(GameWindowEvent::Resized(size));
                        }
                    }
                    AndroidMainEvent::Destroy => {
                        terminated = true;
                    }
                    _ => {}
                }
            }
        });

        if terminated {
            // exit before processing events
            break;
        }

        if let Ok(mut iter) = app.input_events_iter() {
            let mut translated_events = Vec::new();

            loop {
                let event_procesed = iter.next(|event| {
                    let mut status = AndroidInputStatus::Unhandled;

                    translate_input_event(
                        &app,
                        game_runtime.game_io().window(),
                        &mut combining_accent,
                        event,
                        |translated| {
                            translated_events.push(translated);
                            status = AndroidInputStatus::Handled;
                        },
                    );

                    for event in translated_events.drain(..) {
                        game_runtime.push_event(event.into());
                    }

                    status
                });

                if !event_procesed {
                    break;
                }
            }
        }
    }

    Ok(())
}
