use crate::async_task::SyncResultAsyncError;
use crate::graphics::*;
use cfg_macros::*;
use std::borrow::Cow;
use std::future::Future;
use std::path::Path;
use std::sync::Arc;

pub trait HasGraphicsContext {
    fn graphics(&self) -> &GraphicsContext;
}

struct GraphicsContextInternal {
    instance: wgpu::Instance,
    adapter: wgpu::Adapter,
    device: wgpu::Device,
    queue: wgpu::Queue,
}

#[derive(Clone)]
pub struct GraphicsContext {
    internal: Arc<GraphicsContextInternal>,
    texture_format: wgpu::TextureFormat,
}

impl HasGraphicsContext for GraphicsContext {
    fn graphics(&self) -> &GraphicsContext {
        self
    }
}

impl GraphicsContext {
    pub fn wgpu_instance(&self) -> &wgpu::Instance {
        &self.internal.instance
    }

    pub fn adapter(&self) -> &wgpu::Adapter {
        &self.internal.adapter
    }

    pub fn device(&self) -> &wgpu::Device {
        &self.internal.device
    }

    pub fn queue(&self) -> &wgpu::Queue {
        &self.internal.queue
    }

    pub fn default_texture_format(&self) -> wgpu::TextureFormat {
        self.texture_format
    }

    pub fn set_default_texture_format(&mut self, format: wgpu::TextureFormat) {
        self.texture_format = format;
    }

    pub fn load_wgsl<P: AsRef<Path> + ?Sized>(
        &self,
        path: &P,
    ) -> std::io::Result<
        SyncResultAsyncError<
            wgpu::ShaderModule,
            wgpu::Error,
            impl Future<Output = Option<wgpu::Error>>,
        >,
    > {
        let wgsl = std::fs::read_to_string(path)?;
        Ok(
            self.load_shader_from_descriptor(wgpu::ShaderModuleDescriptor {
                label: Some(path.as_ref().to_string_lossy().as_ref()),
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(wgsl.as_str())),
            }),
        )
    }

    pub fn load_wgsl_from_str(
        &self,
        source: &str,
    ) -> SyncResultAsyncError<
        wgpu::ShaderModule,
        wgpu::Error,
        impl Future<Output = Option<wgpu::Error>>,
    > {
        self.load_shader_from_descriptor(wgpu::ShaderModuleDescriptor {
            label: None,
            source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(source)),
        })
    }

    pub fn load_shader_from_descriptor(
        &self,
        descriptor: wgpu::ShaderModuleDescriptor,
    ) -> SyncResultAsyncError<
        wgpu::ShaderModule,
        wgpu::Error,
        impl Future<Output = Option<wgpu::Error>>,
    > {
        let device = self.device();

        device.push_error_scope(wgpu::ErrorFilter::Validation);
        let shader = device.create_shader_module(descriptor);
        let error = device.pop_error_scope();

        SyncResultAsyncError::new(shader, error)
    }

    pub async fn new(
        instance: wgpu::Instance,
        surface: Option<&wgpu::Surface>,
    ) -> anyhow::Result<GraphicsContext> {
        let adapter_opt = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::HighPerformance,
                compatible_surface: surface,
                force_fallback_adapter: false,
            })
            .await;

        let adapter = adapter_opt.ok_or_else(|| anyhow::anyhow!("No adapter found"))?;

        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: None,
                    limits: {
                        cfg_web! {
                            wgpu::Limits::downlevel_webgl2_defaults()
                        }
                        cfg_native! {
                            wgpu::Limits::default()
                        }
                    },
                    features: wgpu::Features::empty(),
                },
                None,
            )
            .await?;

        Ok(GraphicsContext {
            internal: Arc::new(GraphicsContextInternal {
                instance,
                adapter,
                device,
                queue,
            }),
            texture_format: wgpu::TextureFormat::Rgba8UnormSrgb,
        })
    }
}
