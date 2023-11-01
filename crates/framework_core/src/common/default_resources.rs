use crate::common::GameIO;
use crate::graphics::*;

pub(crate) fn inject(game_io: &mut GameIO) {
    game_io.set_resource(CopyPipeline::new(game_io));
    game_io.set_resource(FlatPipeline::new(game_io));
    game_io.set_resource(DefaultSpritePipeline::new(game_io));
    game_io.set_resource(DefaultSpriteSampler::new(game_io));
    game_io.set_resource(DefaultSpriteMesh::new(game_io));
    game_io.set_resource(DefaultSpriteMeshInverted::new(game_io));
    game_io.set_resource(DefaultTextureSourceMesh::new(game_io));
}
