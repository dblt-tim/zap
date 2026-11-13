use wgpu::{Adapter, Device, DeviceDescriptor, ExperimentalFeatures, Features, Limits, MemoryHints, Queue, Trace};


pub async fn get_device_and_queue(adapter: &Adapter) -> anyhow::Result<(Device, Queue)> {

    let desc = DeviceDescriptor {
        label: None,
        required_features: Features::empty(),
        required_limits: Limits::default(),
        experimental_features: ExperimentalFeatures::disabled(),
        memory_hints: MemoryHints::default(),
        trace: Trace::Off
    };

    let device_queue = adapter.request_device(&desc).await?;

    return Ok(device_queue);
}