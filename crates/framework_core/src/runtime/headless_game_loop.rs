use super::HeadlessGameWindow;
use crate::runtime::{GameRuntimeCore, GameRuntimeCoreParams, GameWindowConfig, GameWindowLoop};
use std::future::Future;

/// A game loop without a window
pub struct HeadlessGameLoop {
    window: HeadlessGameWindow,
}

impl GameWindowLoop for HeadlessGameLoop {
    type PlatformApp = ();

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

impl HeadlessGameLoop {
    async fn build(window_config: GameWindowConfig<()>) -> anyhow::Result<HeadlessGameLoop> {
        let window = HeadlessGameWindow::from_config(window_config).await?;

        let window_loop = Self { window };

        Ok(window_loop)
    }

    async fn run(self, runtime_params: GameRuntimeCoreParams) -> anyhow::Result<()> {
        let window = Box::new(self.window);

        let mut game_runtime = GameRuntimeCore::new(window, runtime_params)?;

        while !game_runtime.quitting() {
            game_runtime.tick();
            game_runtime.sleep().await;
        }

        Ok(())
    }
}
