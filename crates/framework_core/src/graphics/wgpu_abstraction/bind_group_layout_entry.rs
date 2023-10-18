#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct BindGroupLayoutEntry {
    pub visibility: wgpu::ShaderStages,
    pub binding_type: wgpu::BindingType,
}
