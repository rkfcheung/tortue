# Tortue

Greetings, coding adventurer! Welcome to **Tortue** (pronounce "tor-too")—a delightful turtle crafted in Rust, a tremendously cool programming language. If you've tinkered with Logo on [Turtle Academy](https://turtleacademy.com), you'll be chuffed to bits with this! Tortue is like your very own digital pet turtle that can draw squares, circles, and all manner of shapes on the screen.

## What's Tortue All About?

- **Drawing**: Instruct Tortue to move forward, turn right, or lift its pen—watch it doodle!
- **Rust Power**: Rust is lightning-fast and secure, perfect for creating games and applications.
- **Your Ideas**: Fancy a turtle that dances or draws stars? You can make it happen!

## Why Did We Make It?

We're building a brand new version from scratch—simple, enjoyable, and crash-free! It's like Logo but with Rust's marvellous magic.

## What's Coming?

- Move Tortue with commands like `forward 100` or `right 90`.
- Draw shapes with brilliant colours.
- Steer it with arrow keys, just like a game!
- Perhaps even type Logo words to direct it about.

## How to Play?

1. Open a terminal (that black box with text).
2. Type `cargo run` to wake Tortue up.
3. Watch it draw (coming soon—we're starting small!).
4. Add the following to your `Cargo.toml`:

```toml
[dependencies]
macroquad = "0.4"
tortue = { git = "https://github.com/rkfcheung/tortue", branch = "main" }
```

## How to Start?

For example, draw a [circle](examples/src/circle.rs).

```rust
use tortue::prelude::*;
#[macroquad::main("Tortue")]
async fn main() {
    // Create a new tortue instance
    let mut tortue = Tortue::new();
    // Draw a circle by iterating 360 times
    for _ in 0..360 {
        // Move forward three steps
        tortue.forward(2.0);
        // Rotate to the right (clockwise) by 1 degree
        tortue.right(1.0);
    }
    // Draw the tortue's path
    tortue.draw().await;
}
```

More examples [here](examples/src/).

Stick around—this is just the beginning! Got ideas? Tell us—we're making Tortue together!

Happy coding!

**## Acknowledgement**

This project is inspired by [`turtle-rs`](https://turtle.rs), and we're re-implementing it using [Macroquad](https://macroquad.rs).
