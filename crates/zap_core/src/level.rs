
pub struct Level;

pub enum LevelLoader {
    FromFunc(fn() -> Level),
}