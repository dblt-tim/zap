use wgpu::{
    Adapter, BackendOptions, Backends, Instance, InstanceDescriptor, InstanceFlags, MemoryBudgetThresholds, PowerPreference, RequestAdapterOptions, Surface,
};

pub fn create_instance() -> Instance {
    let descriptor = InstanceDescriptor {
        backends: Backends::PRIMARY,
        flags: InstanceFlags::default(),
        memory_budget_thresholds: MemoryBudgetThresholds::default(),
        backend_options: BackendOptions::default(),
        display: None,
    };

    let instance = Instance::new(descriptor);

    return instance;
}

pub async fn get_instance_adapter(instance: &Instance, surface: &Surface<'_>) -> anyhow::Result<Adapter> {

    let options = RequestAdapterOptions {
        power_preference: PowerPreference::default(),
        force_fallback_adapter: false,
        compatible_surface: Some(surface),
        apply_limit_buckets: true,
    };

    let adapter = instance.request_adapter(&options).await?;

    return Ok(adapter);
}