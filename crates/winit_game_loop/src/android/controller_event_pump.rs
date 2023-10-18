use framework_core::runtime::GameRuntimeCore;

pub(crate) struct ControllerEventPump {}

impl ControllerEventPump {
    pub(crate) fn new(game_runtime: &mut GameRuntimeCore) -> anyhow::Result<Self> {
        Ok(Self {})
    }

    pub(crate) fn pump(&mut self, game_runtime: &mut GameRuntimeCore) {}
}
