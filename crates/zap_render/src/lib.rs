use std::sync::Arc;

use winit::window::Window;

use anyhow::Result;

mod instance;
use instance::*;

mod surface;
use surface::*;

mod device;
use device::*;

pub struct Renderer {
    window: Arc<Window>,

    surface: wgpu::Surface<'static>,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    is_surface_configured: bool,
}

impl Renderer {
    pub async fn new(window: Arc<Window>) -> Result<Self> {
        let size = window.inner_size();

        let instance = create_instance();

        let surface = instance.create_surface(window.clone()).unwrap();

        let adapter = get_instance_adapter(&instance, &surface).await?;

        let (device, queue) = get_device_and_queue(&adapter).await?;

        let surf_caps = surface.get_capabilities(&adapter);
        let format = surf_caps
            .formats
            .iter()
            .find(|f| f.is_srgb())
            .copied()
            .unwrap_or(surf_caps.formats[0]);

        let config = make_surface_config(format, size, &surf_caps);

        Ok(Self {
            window,
            surface,
            device,
            queue,
            config,
            is_surface_configured: false,
        })
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        if width > 0 && height > 0 {
            self.config.width = width;
            self.config.height = height;
            self.surface.configure(&self.device, &self.config);
            self.is_surface_configured = true;
        }
    }

    pub fn render(&mut self) -> Result<()> {
        self.window.request_redraw();

        if !self.is_surface_configured {
            return Ok(());
        }

        type CST = wgpu::CurrentSurfaceTexture;

        let output = match self.surface.get_current_texture() {
            CST::Success(surf_tex) => surf_tex,
            CST::Suboptimal(surf_tex) => surf_tex,
            CST::Timeout | CST::Occluded | CST::Validation => return Ok(()),
            CST::Outdated => {
                self.surface.configure(&self.device, &self.config);
                return Ok(());
            }
            CST::Lost => {
                anyhow::bail!("Lost device...");
            }
        };

        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });

        let render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("Render Pass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: &view,
                resolve_target: None,
                depth_slice: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color {
                        r: 0.1,
                        g: 0.2,
                        b: 0.3,
                        a: 1.0,
                    }),
                    store: wgpu::StoreOp::Store,
                },
            })],
            depth_stencil_attachment: None,
            occlusion_query_set: None,
            timestamp_writes: None,
            multiview_mask: None,
        });

        drop(render_pass);

        self.queue.submit(std::iter::once(encoder.finish()));
        self.queue.present(output);

        Ok(())
    }
}
