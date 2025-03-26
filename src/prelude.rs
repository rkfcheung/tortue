use macroquad::{
    math::Vec2,
    window::{screen_height, screen_width},
};

pub use crate::tortue::Tortue;

pub type Angle = f32;
pub type Distance = f32;
pub type Point = Vec2;
pub type Speed = usize;

pub fn origin() -> Point {
    (screen_width() / 2.0, screen_height() / 2.0).into()
}
