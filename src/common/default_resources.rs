use crate::prelude::*;

pub(super) fn inject(game_io: &mut GameIO) {
    game_io.set_resource(FlatPipeline::new(game_io));
    game_io.set_resource(DefaultSpritePipeline::new(game_io));
    game_io.set_resource(DefaultSpriteSampler::new(game_io));
    game_io.set_resource(DefaultSpriteMesh::new());
    game_io.set_resource(DefaultSpriteMeshInverted::new());
}
