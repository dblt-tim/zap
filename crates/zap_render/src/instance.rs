use anyhow::Result;

pub fn create_instance() -> wgpu::Instance {
    let desc = wgpu::InstanceDescriptor {
        backends: wgpu::Backends::PRIMARY,
        flags: Default::default(),
        memory_budget_thresholds: Default::default(),
        backend_options: Default::default(),
        display: None,
    };

    let instance = wgpu::Instance::new(desc);
    return instance;
}

pub async fn get_instance_adapter(
    instance: &wgpu::Instance,
    surface: &wgpu::Surface<'static>,
) -> Result<wgpu::Adapter> {
    let opt = wgpu::RequestAdapterOptions {
        power_preference: wgpu::PowerPreference::default(),
        compatible_surface: Some(surface),
        force_fallback_adapter: false,
        apply_limit_buckets: true,
    };

    let adapter = instance.request_adapter(&opt).await?;

    return Ok(adapter);
}
