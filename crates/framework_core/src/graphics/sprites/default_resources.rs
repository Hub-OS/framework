use crate::common::GameIO;
use crate::graphics::*;
use std::sync::Arc;

/// A wrapped SpritePipeline for storage in resources. Preset and accessible from GameIO::resource()
pub struct DefaultSpritePipeline {
    shader: wgpu::ShaderModule,
    pipeline: SpritePipeline<SpriteInstanceData>,
}

impl DefaultSpritePipeline {
    pub(crate) fn new(game_io: &GameIO) -> Self {
        let device = game_io.graphics().device();

        let shader = device.create_shader_module(include_wgsl!("sprite_shader.wgsl"));

        let render_pipeline = RenderPipelineBuilder::new(game_io)
            .with_uniform_bind_group(&[BindGroupLayoutEntry {
                visibility: wgpu::ShaderStages::VERTEX,
                binding_type: OrthoCamera::binding_type(),
            }])
            .with_instance_bind_group(
                SpritePipeline::<SpriteInstanceData>::instance_bind_group_layout(),
            )
            .with_vertex_shader(&shader, "vs_main")
            .with_fragment_shader(&shader, "fs_main")
            .build::<SpriteVertex, SpriteInstanceData>()
            .unwrap();

        Self {
            shader,
            pipeline: SpritePipeline::from_custom_pipeline(render_pipeline),
        }
    }

    pub fn shader_module(&self) -> &wgpu::ShaderModule {
        &self.shader
    }

    pub fn as_sprite_pipeline(&self) -> &SpritePipeline<SpriteInstanceData> {
        &self.pipeline
    }
}

/// A wrapped Arc<TextureSampler> for storage in resources. Preset and accessible from GameIO::resource()
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

pub struct DefaultSpriteMesh {
    mesh: Arc<Mesh<SpriteVertex>>,
}

impl DefaultSpriteMesh {
    pub(crate) fn new(game_io: &GameIO) -> Self {
        Self {
            mesh: Self::create_mesh(game_io, false),
        }
    }

    fn create_mesh(graphics: &impl HasGraphicsContext, invert_y: bool) -> Arc<Mesh<SpriteVertex>> {
        let (y1, y2) = if invert_y { (0.0, 1.0) } else { (1.0, 0.0) };

        Mesh::new(
            graphics,
            &[
                SpriteVertex {
                    vertex: [0.0, 0.0],
                    uv: [0.0, y1],
                },
                SpriteVertex {
                    vertex: [0.0, 1.0],
                    uv: [0.0, y2],
                },
                SpriteVertex {
                    vertex: [1.0, 1.0],
                    uv: [1.0, y2],
                },
                SpriteVertex {
                    vertex: [1.0, 0.0],
                    uv: [1.0, y1],
                },
            ],
            &[0, 1, 2, 2, 0, 3],
        )
    }

    pub fn as_mesh(&self) -> &Arc<Mesh<SpriteVertex>> {
        &self.mesh
    }
}

pub struct DefaultSpriteMeshInverted {
    mesh: Arc<Mesh<SpriteVertex>>,
}

impl DefaultSpriteMeshInverted {
    pub(crate) fn new(game_io: &GameIO) -> Self {
        Self {
            mesh: DefaultSpriteMesh::create_mesh(game_io, true),
        }
    }

    pub fn as_mesh(&self) -> &Arc<Mesh<SpriteVertex>> {
        &self.mesh
    }
}
