use macroquad::prelude::*;
use tortue::prelude::*;

#[macroquad::main("Tortue")]
async fn main() {
    let mut tortue = Tortue::new();
    tortue.set_color(BLUE);

    for _ in 0..360 {
        tortue.forward(1.0);
        tortue.right(1.0);
    }

    tortue.set_fill_color(GREEN); // Fill with green
    tortue.begin_fill();
    tortue.forward(100.0); // Right
    tortue.right(120.0); // Turn
    tortue.forward(100.0); // Diagonal
    tortue.right(120.0); // Turn
    tortue.forward(100.0); // Back
    tortue.end_fill();

    tortue.draw().await;
}
