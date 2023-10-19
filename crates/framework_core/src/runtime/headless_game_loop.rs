use super::HeadlessGameWindow;
use crate::runtime::{GameRuntimeCore, GameRuntimeCoreParams, GameWindowConfig, GameWindowLoop};
use std::future::Future;

/// A game loop without window and input
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

        let mut game_runtime = GameRuntimeCore::new(window, runtime_params).await?;

        let game_io = game_runtime.game_io_mut();
        game_io.input_mut().set_default_controller_deadzone(0.05);

        while !game_runtime.quitting() {
            game_runtime.tick();
            game_runtime.sleep().await;
        }

        Ok(())
    }
}
