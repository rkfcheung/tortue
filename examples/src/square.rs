// Ported from https://github.com/sunjay/turtle/blob/master/examples/square.rs

use tortue::prelude::*;

#[macroquad::main("Tortue")]
async fn main() {
    let mut tortue = Tortue::new();

    for _ in 0..4 {
        tortue.forward(200.0);
        tortue.right(90.0);
    }

    tortue.draw().await;
}
