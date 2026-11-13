use winit::{
    application::ApplicationHandler, event::{KeyEvent, WindowEvent}, event_loop::{ActiveEventLoop, EventLoop}, keyboard::PhysicalKey, window::Window,
};

use crate::state::State;

use std::sync::Arc;

pub struct App {
    state: Option<State>,
}
impl App {
    pub fn new() -> Self {
        Self { state: None }
    }

    pub fn run() -> anyhow::Result<()> {
        env_logger::init();

        let event_loop = EventLoop::with_user_event().build()?;

        let mut app = App::new();
        event_loop.run_app(&mut app)?;

        Ok(())
    }
}
impl ApplicationHandler<State> for App {
    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: winit::window::WindowId,
        event: WindowEvent,
    ) {
        let state = match &mut self.state {
            Some(canvas) => canvas,
            None => return,
        };

        type WE = WindowEvent;

        match event {
            WE::CloseRequested => event_loop.exit(),
            WE::Resized(size) => state.resize(size.width, size.height),
            WE::RedrawRequested => { 
                state.update();
                
                match state.render() {
                    Ok(_) => {},
                    Err(e) => {
                        log::error!("{e}");
                        event_loop.exit();
                    }
                } 
            },
            WE::KeyboardInput { 
                event: KeyEvent {
                    physical_key: PhysicalKey::Code(code),
                    state: key_state,
                    ..
                },
                ..
            } => state.handle_key(event_loop, code, key_state.is_pressed()),
            _ => {}
        }
    }

    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        #[allow(unused_mut)]
        let mut window_attributes = Window::default_attributes();

        let window = Arc::new(event_loop.create_window(window_attributes).unwrap());

        self.state = Some(pollster::block_on(State::new(window)).unwrap());
    }

    fn user_event(&mut self, _event_loop: &ActiveEventLoop, event: State) {
        self.state = Some(event);
    }
}
