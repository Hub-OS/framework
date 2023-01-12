use crate::prelude::*;

pub struct SpritePipeline<SpriteData: InstanceData> {
    render_pipeline: RenderPipeline<SpriteVertex, SpriteData>,
}

impl<SpriteData: InstanceData> SpritePipeline<SpriteData> {
    pub(crate) fn new(game_io: &GameIO) -> Self {
        let device = game_io.graphics().device();

        let shader = device.create_shader_module(include_wgsl!("sprite_shader.wgsl"));

        let render_pipeline = RenderPipelineBuilder::new(game_io)
            .with_uniform_bind_group(vec![OrthoCamera::bind_group_layout_entry(0)])
            .with_instance_bind_group_layout(vec![
                Texture::bind_group_layout_entry(0),
                TextureSampler::bind_group_layout_entry(1),
            ])
            .with_vertex_shader(&shader, "vs_main")
            .with_fragment_shader(&shader, "fs_main")
            .build::<SpriteVertex, SpriteData>()
            .unwrap();

        Self { render_pipeline }
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
