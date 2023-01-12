use crate::prelude::*;
use std::sync::Arc;

pub struct DefaultSpritePipeline {
    pipeline: SpritePipeline<SpriteInstanceData>,
}

impl DefaultSpritePipeline {
    pub(crate) fn new(game_io: &GameIO) -> Self {
        Self {
            pipeline: SpritePipeline::new(game_io),
        }
    }

    pub fn as_sprite_pipeline(&self) -> &SpritePipeline<SpriteInstanceData> {
        &self.pipeline
    }
}

pub struct DefaultSpriteSampler {
    sampler: Arc<TextureSampler>,
}

impl DefaultSpriteSampler {
    pub(crate) fn new(game_io: &GameIO) -> Self {
        Self {
            sampler: TextureSampler::new(game_io, SamplingFilter::Nearest, EdgeSampling::Clamp),
        }
    }

    pub fn as_texture_sampler(&self) -> &Arc<TextureSampler> {
        &self.sampler
    }
}
