use macroquad::prelude::*;

mod tortue;  // Tell Rust to use tortue.rs
use tortue::Tortue;

#[macroquad::main("Tortue")]
async fn main() {
    let mut tortue = Tortue::new();
    loop {
        clear_background(WHITE);
        draw_circle(tortue.x, tortue.y, 5.0, RED);  // Show tortue as a red dot
        next_frame().await;
    }
}