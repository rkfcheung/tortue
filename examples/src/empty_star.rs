// Ported from https://github.com/sunjay/turtle/blob/master/examples/empty_star.rs

use macroquad::prelude::*;
use tortue::prelude::*;

#[macroquad::main("Tortue")]
async fn main() {
    let mut tortue = Tortue::new();

    let points = 5.0;
    let angle = 180.0 / points;

    tortue.set_pen_size(4.0);
    tortue.set_pen_color(YELLOW);

    tortue.pen_up();
    tortue.forward(150.0);
    tortue.right(180.0 - angle / 2.0);
    tortue.pen_down();

    for _ in 0..5 {
        tortue.forward(100.0);
        tortue.left(angle * 2.0);
        tortue.forward(100.0);
        tortue.right(180.0 - angle);
    }

    tortue.draw().await;
}
