use macroquad::prelude::*;

pub struct Tortue {
    x: f32,
    y: f32,
    angle: f32,
}

impl Tortue {
    pub fn new() -> Self {
        Tortue {
            x: 400.0, // Centre of 800x600 window
            y: 300.0,
            angle: 0.0, // Facing right (0 degrees)
        }
    }

    pub fn forward(&mut self, distance: f32) {
        let rad = self.angle.to_radians(); // Convert degrees to radians
        let new_x = self.x + distance * rad.cos();
        let new_y = self.y - distance * rad.sin(); // Y goes down in graphics
        draw_line(self.x, self.y, new_x, new_y, 2.0, BLACK); // Draw the path
        self.x = new_x;
        self.y = new_y;
    }

    pub fn start(&self) {
        clear_background(WHITE);
        draw_circle(self.x, self.y, 5.0, RED);
    }
}
