use super::{GameRuntimeCoreParams, GameWindowConfig};
use std::future::Future;

pub trait GameWindowLoop: Sized {
    type PlatformApp;

    fn build(window_config: GameWindowConfig<Self::PlatformApp>) -> anyhow::Result<Self>;

    fn run(self, params: GameRuntimeCoreParams) -> Box<dyn Future<Output = anyhow::Result<()>>>;
}
