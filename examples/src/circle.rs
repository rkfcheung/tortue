// Ported from https://github.com/sunjay/turtle/blob/master/examples/circle.rs

use tortue::prelude::*;

#[macroquad::main("Tortue")]
async fn main() {
    // Create a new tortue instance
    let mut tortue = Tortue::new();

    // Draw a circle by iterating 360 times
    for _ in 0..360 {
        // Move forward three steps
        tortue.forward(2.0);
        // Rotate to the right (clockwise) by 1 degree
        tortue.right(1.0);
    }

    // Draw the tortue's path
    tortue.draw().await;
}
