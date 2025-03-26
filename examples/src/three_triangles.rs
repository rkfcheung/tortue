// Ported from https://github.com/sunjay/turtle/blob/master/examples/three-triangles.rs

use tortue::prelude::*;

#[macroquad::main("Tortue")]
async fn main() {
    let mut tortue = Tortue::new();

    // Rotate the whole image by 30 degrees
    tortue.left(30.0);

    // Draw three triangles
    for _ in 0..3 {
        // Draw one triangle
        for _ in 0..3 {
            tortue.forward(100.0);
            tortue.right(120.0);
        }

        // Rotate the turtle so that the triangles
        // aren't drawn over each other
        tortue.left(120.0);
    }

    tortue.draw().await;
}
