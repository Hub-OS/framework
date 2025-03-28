use crate::graphics::*;
use math::*;

/// "RenderPasses only render when flushed"
pub struct RenderPass<'a> {
    encoder: &'a mut wgpu::CommandEncoder,
    label: Option<&'static str>,
    color_targets: Vec<&'a RenderTarget>,
    depth_target: Option<&'a RenderTarget>,
    queues: Vec<Vec<RenderOperation>>,
    texture_size: UVec2,
    clear_color: Option<Color>,
}

impl<'a> RenderPass<'a> {
    pub fn new(encoder: &'a mut wgpu::CommandEncoder, color_target: &'a RenderTarget) -> Self {
        Self {
            encoder,
            label: None,
            color_targets: vec![color_target],
            depth_target: None,
            queues: Vec::new(),
            texture_size: color_target.size(),
            clear_color: color_target.clear_color(),
        }
    }

    pub fn with_additional_color_target(mut self, color_target: &'a RenderTarget) -> Self {
        self.color_targets.push(color_target);
        self
    }

    pub fn with_depth_target(mut self, depth_target: &'a RenderTarget) -> Self {
        self.depth_target = Some(depth_target);
        self
    }

    pub fn target_size(&self) -> UVec2 {
        self.texture_size
    }

    pub fn clear_color(&self) -> Option<Color> {
        self.clear_color
    }

    pub fn create_subpass<'b>(&'b mut self, color_target: &'b RenderTarget) -> RenderPass<'b> {
        RenderPass {
            encoder: self.encoder,
            label: Some("render_target_pass"),
            color_targets: vec![color_target],
            depth_target: None,
            queues: Vec::new(),
            texture_size: color_target.size(),
            clear_color: color_target.clear_color(),
        }
    }

    pub fn consume_queue<RenderQueue: RenderQueueTrait>(&mut self, queue: RenderQueue) {
        let operations = queue.into_operation_vec();

        if !operations.is_empty() {
            self.queues.push(operations);
        }
    }

    pub fn flush(self) {
        let color_attachments: Vec<_> = self
            .color_targets
            .iter()
            .map(|target| Some(target.color_attachment()))
            .collect();

        let depth_stencil_attachment = self.depth_target.map(|target| target.depth_attachment());

        let descriptor = wgpu::RenderPassDescriptor {
            label: self.label,
            color_attachments: &color_attachments,
            depth_stencil_attachment,
            occlusion_query_set: None,
            timestamp_writes: None,
        };

        let encoder = self.encoder;
        let mut render_pass = encoder.begin_render_pass(&descriptor);

        for queue in &self.queues {
            // println!("RenderOperations: {}", queue.len());

            let mut scissor_set = false;

            for operation in queue {
                match operation {
                    RenderOperation::SetPipeline(render_pipeline) => {
                        render_pass.set_pipeline(render_pipeline);
                        // println!("set pipeline");
                    }
                    RenderOperation::SetScissor(rect) => {
                        let rect = rect.scissor(Rect::UNIT) * self.texture_size.as_vec2();

                        // rounding to avoid precision issues, applying a minimum size to avoid wgpu complaints
                        render_pass.set_scissor_rect(
                            (rect.x.round() as u32).min(self.texture_size.x - 1),
                            (rect.y.round() as u32).min(self.texture_size.y - 1),
                            (rect.width.round() as u32).max(1),
                            (rect.height.round() as u32).max(1),
                        );

                        scissor_set = true;
                        // println!("set scissor");
                    }
                    RenderOperation::SetUniforms(bind_group) => {
                        render_pass.set_bind_group(0, bind_group, &[]);
                        // println!("set uniforms");
                    }
                    RenderOperation::SetMesh((vertex_buffer, index_buffer)) => {
                        render_pass.set_vertex_buffer(0, vertex_buffer.slice(..));
                        render_pass
                            .set_index_buffer(index_buffer.slice(..), wgpu::IndexFormat::Uint32);
                        // println!("change mesh");
                    }
                    RenderOperation::SetInstanceResources(bind_group) => {
                        render_pass.set_bind_group(1, bind_group, &[]);
                        // println!("set instance resources");
                    }
                    RenderOperation::Draw {
                        instance_buffer,
                        index_count,
                        instance_count,
                    } => {
                        if let Some(instance_buffer) = instance_buffer {
                            render_pass.set_vertex_buffer(1, instance_buffer.slice(..));
                        }

                        render_pass.draw_indexed(0..*index_count, 0, 0..*instance_count);
                        // println!("draw");
                    }
                }
            }

            if scissor_set {
                render_pass.set_scissor_rect(0, 0, self.texture_size.x, self.texture_size.y);
            }
        }
    }
}
