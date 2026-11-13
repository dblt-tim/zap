use winit::{event_loop::ActiveEventLoop, keyboard::KeyCode, window::Window};

use std::sync::Arc;

use crate::rendering::Renderer;

pub struct State {
    renderer: Renderer,
    window: Arc<Window>,
}
impl State {
    pub async fn new(window: Arc<Window>) -> anyhow::Result<Self> {
        let renderer = Renderer::init(window.clone()).await?;

        Ok(Self { renderer, window })
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        self.renderer.resize(width, height);
    }

    pub fn render(&mut self) -> anyhow::Result<()> {
        self.window.request_redraw();

        self.renderer.render()?;

        Ok(())
    }

    pub fn handle_key(&self, event_loop: &ActiveEventLoop, code: KeyCode, is_pressed: bool) {
        match (code, is_pressed) {
            (KeyCode::Escape, true) => event_loop.exit(),
            _ => {}
        }
    }

    pub fn update(&mut self) {}
}
