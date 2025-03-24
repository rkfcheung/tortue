pub struct Tortue {
    pub x: f32,
    pub y: f32,
    pub angle: f32,
}

impl Tortue {
    pub fn new() -> Self {
        Tortue {
            x: 400.0, // Centre of 800x600 window
            y: 300.0,
            angle: 0.0, // Facing right (0 degrees)
        }
    }
}
