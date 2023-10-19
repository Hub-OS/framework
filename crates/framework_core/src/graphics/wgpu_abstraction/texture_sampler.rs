use crate::graphics::*;
use std::sync::Arc;

pub enum SamplingFilter {
    Linear,
    Nearest,
}

pub enum EdgeSampling {
    Clamp,
    Repeat,
}

pub struct TextureSampler {
    sampler: wgpu::Sampler,
}

impl TextureSampler {
    pub fn new(
        graphics: &impl HasGraphicsContext,
        sampling_filter: SamplingFilter,
        edge_sampling: EdgeSampling,
    ) -> Arc<Self> {
        let address_mode = match edge_sampling {
            EdgeSampling::Clamp => wgpu::AddressMode::ClampToEdge,
            EdgeSampling::Repeat => wgpu::AddressMode::Repeat,
        };

        let zoom_filter = match sampling_filter {
            SamplingFilter::Linear => wgpu::FilterMode::Linear,
            SamplingFilter::Nearest => wgpu::FilterMode::Nearest,
        };

        let device = graphics.graphics().device();

        Arc::new(Self {
            sampler: device.create_sampler(&wgpu::SamplerDescriptor {
                address_mode_u: address_mode,
                address_mode_v: address_mode,
                address_mode_w: address_mode,
                mag_filter: zoom_filter,
                min_filter: zoom_filter,
                ..Default::default()
            }),
        })
    }
}

impl AsBinding for TextureSampler {
    fn as_binding(&self) -> BindingResource {
        wgpu::BindingResource::Sampler(&self.sampler)
    }
}
