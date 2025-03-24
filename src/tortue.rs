use macroquad::prelude::*;

pub struct Tortue {
    x: f32,
    y: f32,
    angle: f32,
    points: Vec<(f32, f32)>, // List of positions
}

impl Tortue {
    pub fn new() -> Self {
        let mut points = Vec::new();
        points.push((400.0, 300.0)); // Starting point
        Tortue {
            x: 400.0, // Centre of 800x600 window
            y: 300.0,
            angle: 0.0, // Facing right (0 degrees)
            points,
        }
    }

    pub fn draw(&self) {
        clear_background(WHITE);
        // Draw all lines connecting points
        for i in 0..self.points.len() - 1 {
            let (x1, y1) = self.points[i];
            let (x2, y2) = self.points[i + 1];
            draw_line(x1, y1, x2, y2, 2.0, BLACK);
        }
        draw_circle(self.x, self.y, 5.0, RED);
    }

    pub fn forward(&mut self, distance: f32) {
        let rad = self.angle.to_radians(); // Convert degrees to radians
        self.x += distance * rad.cos();
        self.y -= distance * rad.sin(); // Y goes down in graphics
        self.points.push((self.x, self.y)); // Add new position
    }
}
