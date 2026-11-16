use std::sync::Arc;

use crate::Level;

use winit::{
    application::ApplicationHandler,
    event::WindowEvent,
    window::{Window, WindowAttributes},
};

pub struct Context {
    level: Option<Level>,
    window: Option<Arc<Window>>, // remove when renderer takes it
    // renderer
}
impl Context {
    pub fn new() -> Self {
        Self {
            level: None,
            window: None,
        }
    }
    pub fn set_level(&mut self, level: Level) {
        self.level = Some(level);
    }
}

impl ApplicationHandler for Context {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        let window = Arc::new(
            event_loop
                .create_window(WindowAttributes::default())
                .unwrap(),
        );

        self.window = Some(window);
    }

    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        _window_id: winit::window::WindowId,
        event: winit::event::WindowEvent,
    ) {
        type WE = WindowEvent;

        match event {
            WE::CloseRequested => event_loop.exit(),

            _ => {}
        }
    }
}
