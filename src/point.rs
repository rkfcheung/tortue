use macroquad::window::{screen_height, screen_width};

#[derive(Clone, Copy)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}

impl Point {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}

impl Default for Point {
    fn default() -> Self {
        Self::new(screen_width() / 2.0, screen_height() / 2.0)
    }
}
