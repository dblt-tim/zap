use wgpu::{Adapter, Surface, SurfaceColorSpace, SurfaceConfiguration, TextureUsages};
use winit::dpi::PhysicalSize;

pub fn make_surface_config(
    surface: &Surface,
    adapter: &Adapter,
    size: PhysicalSize<u32>,
) -> SurfaceConfiguration {
    let surface_caps = surface.get_capabilities(adapter);
    let surface_format = surface_caps
        .formats
        .iter()
        .find(|e| e.is_srgb())
        .copied()
        .unwrap_or(surface_caps.formats[0]);

    let config = SurfaceConfiguration {
        usage: TextureUsages::RENDER_ATTACHMENT,
        format: surface_format,
        color_space: SurfaceColorSpace::Auto,
        width: size.width,
        height: size.height,
        present_mode: surface_caps.present_modes[0],
        desired_maximum_frame_latency: 2,
        alpha_mode: surface_caps.alpha_modes[0],
        view_formats: vec![],
    };

    return config;
}
