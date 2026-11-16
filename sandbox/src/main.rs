
extern crate zap;
use zap::*;

use anyhow::Result;

struct Sandbox;

fn level() -> Level {
    let l = Level;
    return l;
}

impl App for Sandbox {
    fn setup(_ctx: &mut Context) -> LevelLoader {
        return LevelLoader::FromFunc(level);
    }
}

fn main() -> Result<()> {
    zap::run::<Sandbox>(AppConfig {
        window_title: "Sandbox".to_string(),
        window_state: zap::WindowState::Windowed(1280, 720),
    })?;

    Ok(())
}