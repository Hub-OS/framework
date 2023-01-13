use crate::prelude::*;

/// A RenderPipeline for PostProcessing
pub struct PostPipeline {
    render_pipeline: RenderPipeline<Vec2, ()>,
}

impl PostPipeline {
    pub fn new(
        game_io: &GameIO,
        fragment_shader: &wgpu::ShaderModule,
        fragment_entry: &str,
        uniform_bind_group: Vec<wgpu::BindGroupLayoutEntry>,
    ) -> Self {
        let device = game_io.graphics().device();

        let shader = device.create_shader_module(include_wgsl!("post_shader.wgsl"));

        let render_pipeline = RenderPipelineBuilder::new(game_io)
            .with_uniform_bind_group(uniform_bind_group)
            .with_instance_bind_group_layout(vec![
                Texture::bind_group_layout_entry(0),
                TextureSampler::bind_group_layout_entry(1),
            ])
            .with_vertex_shader(&shader, "vs_main")
            .with_fragment_shader(fragment_shader, fragment_entry)
            .build::<Vec2, ()>()
            .unwrap();

        Self { render_pipeline }
    }
}

impl AsRef<RenderPipeline<Vec2, ()>> for PostPipeline {
    fn as_ref(&self) -> &RenderPipeline<Vec2, ()> {
        &self.render_pipeline
    }
}
