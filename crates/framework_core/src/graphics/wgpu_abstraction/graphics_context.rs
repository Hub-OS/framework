use crate::async_task::SyncResultAsyncError;
use crate::graphics::*;
use cfg_macros::*;
use logging::log;
use std::borrow::Cow;
use std::future::Future;
use std::path::Path;

pub trait HasGraphicsContext {
    fn graphics(&self) -> &GraphicsContext;
}

#[derive(Clone)]
pub struct GraphicsContext {
    instance: wgpu::Instance,
    adapter: wgpu::Adapter,
    device: wgpu::Device,
    queue: wgpu::Queue,
    texture_format: wgpu::TextureFormat,
}

impl HasGraphicsContext for GraphicsContext {
    fn graphics(&self) -> &GraphicsContext {
        self
    }
}

impl GraphicsContext {
    pub async fn new(
        instance: wgpu::Instance,
        surface: Option<&wgpu::Surface<'_>>,
    ) -> anyhow::Result<GraphicsContext> {
        log::trace!("Initializing WGPU");

        log::trace!("Requesting Adapter");

        let adapter_opt = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::HighPerformance,
                compatible_surface: surface,
                force_fallback_adapter: false,
            })
            .await;

        let adapter = adapter_opt?;

        log::trace!("Found Adapter: {:#?}", adapter.get_info());

        let required_limits_list = {
            cfg_web! {
                [wgpu::Limits::downlevel_webgl2_defaults()]
            }
            cfg_native! {
                [wgpu::Limits::default(), wgpu::Limits::downlevel_defaults(), wgpu::Limits::downlevel_webgl2_defaults()]
            }
        };

        let mut i = 0;
        let mut last_error: Option<wgpu::RequestDeviceError> = None;

        let (device, queue) = loop {
            let Some(limits) = required_limits_list.get(i) else {
                return Err(last_error.unwrap().into());
            };

            let result = adapter
                .request_device(&wgpu::DeviceDescriptor {
                    label: None,
                    required_limits: limits.clone(),
                    required_features: wgpu::Features::empty(),
                    memory_hints: wgpu::MemoryHints::default(),
                    trace: wgpu::Trace::Off,
                })
                .await;

            match result {
                Ok(tuple) => break tuple,
                Err(err) => last_error = Some(err),
            }

            i += 1;
        };

        log::trace!("WGPU Initialized");

        Ok(GraphicsContext {
            instance,
            adapter,
            device,
            queue,
            texture_format: wgpu::TextureFormat::Rgba8UnormSrgb,
        })
    }

    pub fn wgpu_instance(&self) -> &wgpu::Instance {
        &self.instance
    }

    pub fn adapter(&self) -> &wgpu::Adapter {
        &self.adapter
    }

    pub fn device(&self) -> &wgpu::Device {
        &self.device
    }

    pub fn queue(&self) -> &wgpu::Queue {
        &self.queue
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
}
