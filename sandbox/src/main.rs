use anyhow::Result;
use zap::*;
use zap::core::Context;

struct Sandbox;

impl App for Sandbox {
    fn setup(_ctx: &mut Context) {}
}

fn main() -> Result<()> {
    zap::run::<Sandbox>(AppConfig {
        window_title: "Sandbox".to_string(),
        window_state: zap::WindowState::Windowed(1280, 720),
    })?;

    Ok(())
}
