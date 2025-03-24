use macroquad::prelude::*;

mod tortue;
use tortue::Tortue;

#[macroquad::main("Tortue")]
async fn main() {
    let mut tortue = Tortue::new();
    loop {
        tortue.start(); // Draw the tortue at its current position
        tortue.forward(100.0); // Move forward 100 units
        next_frame().await;
    }
}
