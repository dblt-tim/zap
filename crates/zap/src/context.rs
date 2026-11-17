use std::sync::Arc;

use crate::ecs::World;
use crate::render::Renderer;

use winit::{
    application::ApplicationHandler,
    event::{KeyEvent, WindowEvent},
    keyboard::{KeyCode, PhysicalKey},
    window::WindowAttributes,
};

pub struct Context {
    renderer: Option<Renderer>,
    #[allow(unused)]
    world: World,
}
impl Context {
    pub fn new() -> Self {
        Self {
            renderer: None,
            world: World::new(),
        }
    }
}

impl ApplicationHandler<Renderer> for Context {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        let window = Arc::new(
            event_loop
                .create_window(WindowAttributes::default())
                .unwrap(),
        );

        self.renderer = Some(pollster::block_on(Renderer::new(window)).unwrap());
    }

    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        _window_id: winit::window::WindowId,
        event: winit::event::WindowEvent,
    ) {
        type WE = WindowEvent;

        let r = match &mut self.renderer {
            Some(r) => r,
            None => return,
        };

        match event {
            WE::CloseRequested => event_loop.exit(),
            WE::Resized(size) => r.resize(size.width, size.height),
            WE::RedrawRequested => match r.render() {
                Ok(_) => {}
                Err(e) => {
                    log::error!("{e}");
                    event_loop.exit();
                }
            },
            WE::KeyboardInput {
                event:
                    KeyEvent {
                        physical_key: PhysicalKey::Code(code),
                        state: key_state,
                        ..
                    },
                ..
            } => match (code, key_state.is_pressed()) {
                (KeyCode::Escape, true) => event_loop.exit(),
                _ => {}
            },

            _ => {}
        }
    }
}
