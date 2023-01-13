use crate::prelude::*;

/// A RenderPipeline for rendering FlatModels. Preset and accessible from GameIO::resource()
pub struct FlatPipeline {
    render_pipeline: RenderPipeline<Vec2, FlatInstanceData>,
}

impl FlatPipeline {
    pub(crate) fn new(game_io: &GameIO) -> Self {
        let device = game_io.graphics().device();

        let shader = device.create_shader_module(include_wgsl!("flat_shader.wgsl"));

        let render_pipeline = RenderPipelineBuilder::new(game_io)
            .with_uniform_bind_group(vec![OrthoCamera::bind_group_layout_entry(0)])
            .with_vertex_shader(&shader, "vs_main")
            .with_fragment_shader(&shader, "fs_main")
            .build::<Vec2, FlatInstanceData>()
            .unwrap();

        Self { render_pipeline }
    }
}

impl AsRef<RenderPipeline<Vec2, FlatInstanceData>> for FlatPipeline {
    fn as_ref(&self) -> &RenderPipeline<Vec2, FlatInstanceData> {
        &self.render_pipeline
    }
}
