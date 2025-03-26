use tortue::prelude::*;

#[macroquad::main("Tortue")]
async fn main() {
    let mut tortue = Tortue::new();

    for _ in 0..360 {
        tortue.forward(1.0);
        tortue.right(1.0);
    }

    tortue.draw().await;
}
