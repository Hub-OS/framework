use crate::prelude::*;

/// A RenderPipeline for copying textures. Preset and accessible from GameIO::resource()
pub struct CopyPipeline {
    render_pipeline: RenderPipeline<Vec2, ()>,
}

impl CopyPipeline {
    pub(crate) fn new(game_io: &GameIO) -> Self {
        let device = game_io.graphics().device();

        let shader = device.create_shader_module(include_wgsl!("copy_shader.wgsl"));

        let render_pipeline = RenderPipelineBuilder::new(game_io)
            .with_instance_bind_group_layout(vec![
                Texture::bind_group_layout_entry(0),
                TextureSampler::bind_group_layout_entry(1),
            ])
            .with_vertex_shader(&shader, "vs_main")
            .with_fragment_shader(&shader, "fs_main")
            .build::<Vec2, ()>()
            .unwrap();

        Self { render_pipeline }
    }
}

impl AsRef<RenderPipeline<Vec2, ()>> for CopyPipeline {
    fn as_ref(&self) -> &RenderPipeline<Vec2, ()> {
        &self.render_pipeline
    }
}
