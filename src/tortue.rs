use macroquad::prelude::*;

pub struct Tortue {
    x: f32,
    y: f32,
    angle: f32,
    points: Vec<(f32, f32)>,
    current: usize,
    pen_down: bool,
    color: Color,
}

impl Tortue {
    pub fn new() -> Self {
        Tortue {
            x: screen_width() / 2.0,
            y: screen_height() / 2.0,
            angle: 0.0,
            points: vec![(screen_width() / 2.0, screen_height() / 2.0)],
            current: 0,
            pen_down: true,
            color: BLACK,
        }
    }

    pub fn forward(&mut self, distance: f32) {
        let rad = self.angle.to_radians();
        let new_x = self.x + distance * rad.cos();
        let new_y = self.y + distance * rad.sin();

        if self.pen_down {
            self.points.push((new_x, new_y));
        }

        self.x = new_x;
        self.y = new_y;
    }

    pub fn right(&mut self, degrees: f32) {
        self.angle += degrees;
    }

    pub fn left(&mut self, degrees: f32) {
        self.angle -= degrees;
    }

    pub fn pen_up(&mut self) {
        self.pen_down = false;
    }

    pub fn pen_down(&mut self) {
        self.pen_down = true;
    }

    pub fn set_color(&mut self, color: Color) {
        self.color = color;
    }

    pub async fn draw(&mut self) {
        loop {
            self.update();
            next_frame().await;
        }
    }

    fn update(&mut self) {
        clear_background(WHITE);

        // Gradually increase the number of points drawn
        let ponts_to_draw = self.current;

        // Draw lines and move turtle
        if self.points.len() > 1 {
            for i in 0..ponts_to_draw {
                let (x1, y1) = self.points[i];
                let (x2, y2) = self.points[i + 1];
                draw_line(x1, y1, x2, y2, 2.0, self.color);
            }

            // Update turtle position to the latest drawn point
            if ponts_to_draw > 0 {
                let (new_x, new_y) = self.points[ponts_to_draw];
                self.x = new_x;
                self.y = new_y;
            }
        }

        // Draw turtle position
        draw_circle(self.x, self.y, 5.0, RED);

        // Advance drawing progress
        if self.current < self.points.len() - 1 {
            self.current += 1;
        }
    }
}
