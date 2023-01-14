use crate::prelude::*;

pub struct SpritePipeline<SpriteData: InstanceData> {
    render_pipeline: RenderPipeline<SpriteVertex, SpriteData>,
}

impl<SpriteData: InstanceData> SpritePipeline<SpriteData> {
    pub(crate) fn new(game_io: &GameIO) -> Self {
        let device = game_io.graphics().device();

        let shader = device.create_shader_module(include_wgsl!("sprite_shader.wgsl"));

        let render_pipeline = RenderPipelineBuilder::new(game_io)
            .with_uniform_bind_group(&[BindGroupLayoutEntry {
                visibility: wgpu::ShaderStages::VERTEX,
                binding_type: OrthoCamera::binding_type(),
            }])
            .with_instance_bind_group(Self::instance_bind_group_layout())
            .with_vertex_shader(&shader, "vs_main")
            .with_fragment_shader(&shader, "fs_main")
            .build::<SpriteVertex, SpriteData>()
            .unwrap();

        Self { render_pipeline }
    }

    pub fn instance_bind_group_layout() -> &'static [BindGroupLayoutEntry] {
        &[
            BindGroupLayoutEntry {
                visibility: wgpu::ShaderStages::FRAGMENT,
                binding_type: wgpu::BindingType::Texture {
                    multisampled: false,
                    view_dimension: wgpu::TextureViewDimension::D2,
                    sample_type: wgpu::TextureSampleType::Float { filterable: true },
                },
            },
            BindGroupLayoutEntry {
                visibility: wgpu::ShaderStages::FRAGMENT,
                binding_type: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
            },
        ]
    }

    pub fn from_custom_pipeline(render_pipeline: RenderPipeline<SpriteVertex, SpriteData>) -> Self {
        Self { render_pipeline }
    }
}

impl<SpriteData: InstanceData> AsRef<RenderPipeline<SpriteVertex, SpriteData>>
    for SpritePipeline<SpriteData>
{
    fn as_ref(&self) -> &RenderPipeline<SpriteVertex, SpriteData> {
        &self.render_pipeline
    }
}
