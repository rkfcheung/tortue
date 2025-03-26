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
        Self {
            x: screen_width() / 2.0,
            y: screen_height() / 2.0,
        }
    }
}

impl From<(f32, f32)> for Point {
    fn from(pt: (f32, f32)) -> Self {
        Self { x: pt.0, y: pt.1 }
    }
}

impl From<[f32; 2]> for Point {
    fn from(pt: [f32; 2]) -> Self {
        Self { x: pt[0], y: pt[1] }
    }
}
