// Ported from https://github.com/sunjay/turtle/blob/master/examples/dashed_circle.rs

use tortue::prelude::*;

#[macroquad::main("Tortue")]
async fn main() {
    let mut tortue = Tortue::new();

    for i in 0..360 {
        // `i / 18` will produce 0 for i = 0 to 17, 1 for i = 18 to 35, 2 for i = 36 to 53,
        // 3 for i = 54 to 71, and so on.
        //
        // We then test if that number is even using `% 2`. This tells us whether we're on an even
        // or odd segment of the dashed line. For even segments, the turtle moves forwards, but no
        // line is drawn on the screen.
        if (i / 18) % 2 == 0 {
            tortue.pen_up();
        }

        tortue.forward(2.0);
        tortue.right(1.0);

        if (i / 18) % 2 == 0 {
            tortue.pen_down();
        }
    }

    tortue.draw().await;
}
