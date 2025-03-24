use macroquad::prelude::*;

mod tortue;
use tortue::Tortue;

#[macroquad::main("Tortue")]
async fn main() {
    let mut tortue = Tortue::new();
    tortue.forward(100.0); // Move right
    tortue.forward(100.0); // Move right again
    loop {
        tortue.draw();
        next_frame().await;
    }
}
