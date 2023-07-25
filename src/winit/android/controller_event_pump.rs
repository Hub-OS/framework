use crate::prelude::GameRuntime;

pub(crate) struct ControllerEventPump {}

impl ControllerEventPump {
    pub(crate) fn new(game_runtime: &mut GameRuntime) -> anyhow::Result<Self> {
        Ok(Self {})
    }

    pub(crate) fn pump(&mut self, game_runtime: &mut GameRuntime) {}
}
