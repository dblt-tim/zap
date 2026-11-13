use wgpu::{Device, ShaderModule, ShaderModuleDescriptor, ShaderSource};

pub fn load_shader(device: &Device, source: &str) -> ShaderModule {
    let desc = ShaderModuleDescriptor {
        label: None,
        source: ShaderSource::Wgsl(source.into()),
    };

    let module = device.create_shader_module(desc);

    return module;
}
