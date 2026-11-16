use anyhow::Result;
use winit::event_loop::EventLoop;

use crate::{Context, LevelLoader};

pub trait App {
    fn setup(ctx: &mut Context) -> LevelLoader;
}

pub enum WindowState {
    Fullscreen,
    Windowed(u32, u32),
}

pub struct AppConfig {
    pub window_title: String,
    pub window_state: WindowState,
}

pub fn run<A: App>(_config: AppConfig) -> Result<()> {
    env_logger::init();

    let mut ctx = Context::new();

    match A::setup(&mut ctx) {
        LevelLoader::FromFunc(f) => ctx.set_level(f()),
    }

    let event_loop = EventLoop::with_user_event().build()?;
    event_loop.run_app(&mut ctx)?;
    
    Ok(())
}
