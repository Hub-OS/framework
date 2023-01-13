use crate::prelude::*;

pub trait PostProcess {
    fn render_pipeline(&self) -> &PostPipeline;
    fn uniform_resources(&self) -> Vec<BindingResource>;

    fn update(&mut self, _game_io: &GameIO) {}
}
