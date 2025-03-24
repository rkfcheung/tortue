use macroquad::prelude::*;

#[macroquad::main("Tortue")]
async fn main() {
    loop {
        clear_background(WHITE);
        next_frame().await;
    }
}
