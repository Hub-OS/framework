use super::*;
use crate::loop_states::{RootLoopState, StartingState, StartingStateParams};
use cfg_macros::*;
use framework_core::runtime::{GameRuntimeCoreParams, GameWindowConfig, GameWindowLoop};
use std::future::Future;
use winit::event_loop::EventLoop;

pub struct WinitGameLoop {
    window_config: GameWindowConfig<super::WinitPlatformApp>,
    event_loop: EventLoop<()>,
}

impl GameWindowLoop for WinitGameLoop {
    type PlatformApp = crate::WinitPlatformApp;

    fn run(
        window_config: GameWindowConfig<Self::PlatformApp>,
        runtime_params: GameRuntimeCoreParams,
    ) -> Box<dyn Future<Output = anyhow::Result<()>>> {
        Box::new(async move {
            let game_loop = Self::build(window_config).await?;
            game_loop.run(runtime_params).await
        })
    }
}

impl WinitGameLoop {
    async fn build(
        window_config: GameWindowConfig<crate::WinitPlatformApp>,
    ) -> anyhow::Result<Self> {
        let event_loop = create_winit_event_loop(window_config.platform_app.clone())?;

        let window_loop = Self {
            window_config,
            event_loop,
        };

        Ok(window_loop)
    }

    async fn run(self, params: GameRuntimeCoreParams) -> anyhow::Result<()> {
        let state_params = StartingStateParams {
            window_config: self.window_config,
            runtime_params: params,
        };

        self.event_loop
            .run_app(&mut RootLoopState::new(StartingState::new(state_params)))?;

        Ok(())
    }
}

#[allow(unused_variables)]
fn create_winit_event_loop(
    platform_app: Option<WinitPlatformApp>,
) -> anyhow::Result<EventLoop<()>> {
    cfg_android! {
        if let Some(app) = platform_app {
            use winit::platform::android::EventLoopBuilderExtAndroid;
            use winit::event_loop::EventLoopBuilder;

            return Ok(EventLoopBuilder::new().with_android_app(app).build()?)
        }
    };

    Ok(EventLoop::new()?)
}
