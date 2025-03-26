use crate::prelude::*;
use macroquad::prelude::*;

pub struct Tortue {
    position: Point,
    angle: f32,
    points: Vec<Point>,
    current: usize,
    color: Color,
    pen_color: Color,
    pen_down: bool,
    pen_size: f32,
}

impl Tortue {
    pub fn new() -> Self {
        Tortue {
            position: Point::default(),
            angle: 0.0,
            points: vec![Point::default()],
            current: 0,
            color: RED,
            pen_color: BLACK,
            pen_down: true,
            pen_size: 2.0,
        }
    }

    pub fn arc_left(&mut self, radius: Distance, extent: Angle) {
        for _ in 0..(extent as usize) {
            let rad = self.angle.to_radians();
            let step_x = radius * (rad + std::f32::consts::PI / 2.0).cos();
            let step_y = radius * (rad + std::f32::consts::PI / 2.0).sin();

            if self.pen_down {
                self.points
                    .push((self.position.x + step_x, self.position.y + step_y).into());
            }

            self.left(1.0);
        }
    }

    pub fn arc_right(&mut self, radius: Distance, extent: Angle) {
        for _ in 0..(extent as usize) {
            let rad = self.angle.to_radians();
            let step_x = radius * (rad - std::f32::consts::PI / 2.0).cos();
            let step_y = radius * (rad - std::f32::consts::PI / 2.0).sin();

            if self.pen_down {
                self.points
                    .push((self.position.x + step_x, self.position.y + step_y).into());
            }

            self.right(1.0);
        }
    }

    pub fn backward(&mut self, distance: Distance) {
        let rad = self.angle.to_radians();
        let new_x = self.position.x - distance * rad.cos();
        let new_y = self.position.y - distance * rad.sin();

        if self.pen_down {
            self.points.push((new_x, new_y).into());
        }

        self.set_x(new_x);
        self.set_y(new_y);
    }

    pub fn begin_fill(&mut self) {
        unimplemented!();
    }

    pub fn clear(&mut self) {
        // Reset to initial state
        self.position = Point::default();
        self.angle = 0.0;
        self.points = vec![Point::default()];
        self.current = 0;
        self.pen_down = true;
    }

    pub async fn draw(&mut self) {
        loop {
            self.update();
            next_frame().await;
        }
    }

    pub fn end_fill(&mut self) {
        unimplemented!();
    }

    pub fn fill_color(&self) -> Color {
        unimplemented!();
    }

    pub fn forward(&mut self, distance: Distance) {
        let rad = self.angle.to_radians();
        let new_x = self.position.x + distance * rad.cos();
        let new_y = self.position.y + distance * rad.sin();

        if self.pen_down {
            self.points.push((new_x, new_y).into());
        }

        self.set_x(new_x);
        self.set_y(new_y);
    }

    pub fn go_to<P: Into<Point>>(&mut self, position: P) {
        let new_pos = position.into();

        if self.pen_down {
            self.points.push(new_pos);
        }

        self.set_x(new_pos.x);
        self.set_y(new_pos.y);
    }

    pub fn heading(&self) -> Angle {
        self.angle
    }

    pub fn hide(&mut self) {
        unimplemented!();
    }

    pub fn home(&mut self) {
        self.go_to(Point::default());
        self.angle = 0.0;
    }

    pub fn is_filling(&self) -> bool {
        unimplemented!();
    }

    pub fn is_pen_down(&self) -> bool {
        self.pen_down
    }

    pub fn is_using_degrees(&self) -> bool {
        unimplemented!();
    }

    pub fn is_using_radians(&self) -> bool {
        unimplemented!();
    }

    pub fn is_visible(&self) -> bool {
        unimplemented!();
    }

    pub fn left(&mut self, angle: Angle) {
        self.angle -= angle;
    }

    pub fn pen_color(&self) -> Color {
        self.pen_color
    }

    pub fn pen_down(&mut self) {
        self.pen_down = true;
    }

    pub fn pen_size(&self) -> f32 {
        self.pen_size
    }

    pub fn pen_up(&mut self) {
        self.pen_down = false;
    }

    pub fn position(&self) -> Point {
        self.position
    }

    pub fn reset(&mut self) {
        // Complete reset to initial state
        *self = Tortue::new();
    }

    pub fn right(&mut self, angle: Angle) {
        self.angle += angle;
    }

    pub fn set_fill_color(&mut self, _color: Color) {
        unimplemented!();
    }

    pub fn set_heading(&mut self, angle: Angle) {
        self.angle = angle;
    }

    pub fn set_pen_color(&mut self, color: Color) {
        self.pen_color = color;
    }

    pub fn set_pen_size(&mut self, thickness: f32) {
        self.pen_size = thickness;
    }

    pub fn set_speed<S: Into<Speed>>(&mut self, _speed: S) {
        unimplemented!();
    }

    pub fn set_x(&mut self, x: f32) {
        self.position.x = x;
    }

    pub fn set_y(&mut self, y: f32) {
        self.position.y = y;
    }

    pub fn show(&mut self) {
        unimplemented!();
    }

    pub fn speed(&self) -> Speed {
        unimplemented!();
    }

    pub fn turn_towards<P: Into<Point>>(&mut self, target: P) {
        let target_point = target.into();
        let dx = target_point.x - self.position.x;
        let dy = target_point.y - self.position.y;

        let target_angle = dy.atan2(dx).to_degrees();
        self.set_heading(target_angle);
    }

    pub fn use_degrees(&mut self) {
        unimplemented!();
    }

    pub fn use_radians(&mut self) {
        unimplemented!();
    }

    pub fn wait_for_click(&mut self) {
        unimplemented!();
    }

    pub fn wait(&mut self, _secs: f32) {
        unimplemented!();
    }

    pub fn set_color(&mut self, color: Color) {
        self.color = color;
    }

    fn update(&mut self) {
        clear_background(WHITE);

        // Gradually increase the number of points drawn
        let ponts_to_draw = self.current;

        // Draw lines and move turtle
        if self.points.len() > 1 {
            for i in 0..ponts_to_draw {
                let pt1 = self.points[i];
                let pt2 = self.points[i + 1];
                draw_line(pt1.x, pt1.y, pt2.x, pt2.y, self.pen_size, self.pen_color);
            }

            // Update turtle position to the latest drawn point
            if ponts_to_draw > 0 {
                let new_pt = self.points[ponts_to_draw];
                self.set_x(new_pt.x);
                self.set_y(new_pt.y);
            }
        }

        // Draw turtle position
        draw_circle(self.position.x, self.position.y, 5.0, self.color);

        // Advance drawing progress
        if self.current < self.points.len() - 1 {
            self.current += 1;
        }
    }
}
