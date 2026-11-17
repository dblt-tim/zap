use winit::dpi::PhysicalSize;

pub fn make_surface_config(surf_fmt: wgpu::TextureFormat, size: PhysicalSize<u32>, caps: &wgpu::SurfaceCapabilities) -> wgpu::SurfaceConfiguration {
    let config = wgpu::SurfaceConfiguration {
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
        format: surf_fmt,
        width: size.width,
        height: size.height,
        color_space: wgpu::SurfaceColorSpace::Auto,
        present_mode: caps.present_modes[0],
        alpha_mode: caps.alpha_modes[0],
        desired_maximum_frame_latency: 2,
        view_formats: vec![]
    };

    return config;
}
