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

    tortue.draw().await;
}
