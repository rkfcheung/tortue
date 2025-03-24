use macroquad::prelude::*;

mod tortue;
use tortue::Tortue;

#[macroquad::main("Tortue")]
async fn main() {
    let mut tortue = Tortue::new();
    tortue.forward(100.0); // Right
    tortue.right(90.0); // Turn down
    tortue.forward(100.0); // Down
    tortue.right(90.0); // Turn left
    tortue.forward(100.0); // Left
    tortue.right(90.0); // Turn up
    tortue.forward(100.0); // Up
    loop {
        tortue.draw();
        next_frame().await;
    }
}
