use super::{GameRuntimeCoreParams, GameWindowConfig};
use std::future::Future;

pub trait GameWindowLoop: Sized {
    type PlatformApp;

    fn run(
        window_config: GameWindowConfig<Self::PlatformApp>,
        runtime_params: GameRuntimeCoreParams,
    ) -> Box<dyn Future<Output = anyhow::Result<()>>>;
}
