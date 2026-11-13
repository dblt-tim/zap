mod device;
mod instance;
mod pipeline;
mod shader;
mod surface;

use device::*;
use instance::*;
use pipeline::*;
use surface::*;

use winit::window::Window;

use std::sync::Arc;

pub struct Renderer {
    surface: wgpu::Surface<'static>,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    is_surface_configured: bool,
    render_pipeline: wgpu::RenderPipeline,
}
impl Renderer {
    pub async fn init(window: Arc<Window>) -> anyhow::Result<Self> {
        let win_size = window.inner_size(); // needed for config, window is moved for surface

        let instance = create_instance();

        let surface = instance.create_surface(window).unwrap();

        let adapter = get_instance_adapter(&instance, &surface).await?;

        let (device, queue) = get_device_and_queue(&adapter).await?;

        let config = make_surface_config(&surface, &adapter, win_size);

        let render_pipeline = make_pipeline(&device, &config.format);

        Ok(Self {
            surface,
            device,
            queue,
            config,
            is_surface_configured: false,
            render_pipeline,
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

    pub fn render(&mut self) -> anyhow::Result<()> {
        
        if !self.is_surface_configured {
            return Ok(());
        }

        type CST = wgpu::CurrentSurfaceTexture;

        let output = match self.surface.get_current_texture() {
            CST::Success(surftex) => surftex,
            CST::Suboptimal(surftex) => surftex,
            CST::Timeout | CST::Validation | CST::Occluded => {
                return Ok(());
            }
            CST::Outdated => {
                self.surface.configure(&self.device, &self.config);
                return Ok(());
            }
            CST::Lost => anyhow::bail!("Lost surface..."),
        };

        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });

        let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
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

        render_pass.set_pipeline(&self.render_pipeline);
        render_pass.draw(0..3, 0..1);

        drop(render_pass);

        self.queue.submit(std::iter::once(encoder.finish()));
        self.queue.present(output);

        Ok(())
    }
}
