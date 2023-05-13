use crate::prelude::*;
use raw_window_handle::{HasRawDisplayHandle, HasRawWindowHandle};
use std::any::TypeId;
use std::borrow::Cow;
use std::future::Future;
use std::path::Path;
use std::sync::Arc;

pub struct GraphicsContext {
    surface: wgpu::Surface,
    surface_config: wgpu::SurfaceConfiguration,
    adapter: Arc<wgpu::Adapter>,
    device: Arc<wgpu::Device>,
    queue: Arc<wgpu::Queue>,
    clear_color: Color,
    disabled_post_processes: Vec<TypeId>,
}

impl GraphicsContext {
    pub fn surface(&self) -> &wgpu::Surface {
        &self.surface
    }

    pub fn surface_config(&self) -> &wgpu::SurfaceConfiguration {
        &self.surface_config
    }

    pub fn surface_size(&self) -> UVec2 {
        UVec2::new(self.surface_config.width, self.surface_config.height)
    }

    pub fn adapter(&self) -> &Arc<wgpu::Adapter> {
        &self.adapter
    }

    pub fn device(&self) -> &Arc<wgpu::Device> {
        &self.device
    }

    pub fn queue(&self) -> &Arc<wgpu::Queue> {
        &self.queue
    }

    pub fn clear_color(&self) -> Color {
        self.clear_color
    }

    pub fn set_clear_color(&mut self, color: Color) {
        self.clear_color = color
    }

    pub fn set_post_process_enabled<P: PostProcess + 'static>(&mut self, enabled: bool) {
        let id = TypeId::of::<P>();

        if let Some(index) = self
            .disabled_post_processes
            .iter()
            .position(|stored| *stored == id)
        {
            if enabled {
                self.disabled_post_processes.remove(index);
            }
        } else if !enabled {
            self.disabled_post_processes.push(id);
        }
    }

    pub fn is_post_process_enabled<P: PostProcess + 'static>(&self) -> bool {
        let id = TypeId::of::<P>();

        !self.disabled_post_processes.contains(&id)
    }

    pub(crate) fn internal_is_post_process_enabled(&self, id: TypeId) -> bool {
        !self.disabled_post_processes.contains(&id)
    }

    pub fn set_vsync_enabled(&mut self, enabled: bool) {
        self.surface_config.present_mode = if enabled {
            wgpu::PresentMode::AutoVsync
        } else {
            wgpu::PresentMode::AutoNoVsync
        };

        self.surface.configure(&self.device, &self.surface_config);
    }

    pub fn vsync_enabled(&self) -> bool {
        self.surface_config.present_mode == wgpu::PresentMode::AutoVsync
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
        self.device.push_error_scope(wgpu::ErrorFilter::Validation);
        let shader = self.device.create_shader_module(descriptor);
        let error = self.device.pop_error_scope();

        SyncResultAsyncError::new(shader, error)
    }

    pub(crate) async fn new<Window: HasRawWindowHandle + HasRawDisplayHandle>(
        window: &Window,
        width: u32,
        height: u32,
    ) -> anyhow::Result<GraphicsContext> {
        let instance = {
            // https://github.com/gfx-rs/wgpu/issues/2384
            crate::cfg_android! {
                wgpu::Instance::new(wgpu::Backends::GL)
            }
            crate::cfg_desktop_and_web! {
                wgpu::Instance::default()
            }
        };

        let surface = unsafe { instance.create_surface(window).unwrap() };

        let adapter_opt = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::HighPerformance,
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await;

        let adapter = adapter_opt.ok_or_else(|| anyhow::anyhow!("No adapter found"))?;

        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: None,
                    limits: {
                        crate::cfg_web! {
                            wgpu::Limits::downlevel_webgl2_defaults()
                        }
                        crate::cfg_android! {
                            wgpu::Limits::downlevel_webgl2_defaults()
                        }
                        crate::cfg_desktop! {
                            wgpu::Limits::default()
                        }
                    },
                    features: wgpu::Features::empty(),
                },
                None,
            )
            .await?;

        let mut surface_config = surface
            .get_default_config(&adapter, width, height)
            .expect("Surface unsupported by adapter");
        surface_config.present_mode = wgpu::PresentMode::AutoVsync;

        surface.configure(&device, &surface_config);

        Ok(GraphicsContext {
            surface,
            surface_config,
            adapter: Arc::new(adapter),
            device: Arc::new(device),
            queue: Arc::new(queue),
            clear_color: Color::TRANSPARENT,
            disabled_post_processes: Vec::new(),
        })
    }

    pub(crate) fn resized(&mut self, size: UVec2) {
        self.surface_config.width = size.x;
        self.surface_config.height = size.y;

        self.surface.configure(&self.device, &self.surface_config);
    }
}
