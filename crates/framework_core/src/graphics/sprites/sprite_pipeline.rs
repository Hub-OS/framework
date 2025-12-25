use crate::graphics::*;

#[derive(Clone)]
pub struct SpritePipeline<SpriteData: InstanceData = SpriteInstanceData> {
    render_pipeline: RenderPipeline<SpriteVertex, SpriteData>,
}

impl<SpriteData: InstanceData> SpritePipeline<SpriteData> {
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
