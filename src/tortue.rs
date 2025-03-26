use crate::prelude::*;
use macroquad::prelude::*;

pub struct Tortue {
    position: Point,
    angle: f32,
    points: Vec<Point>,
    current: usize,
    pen_down: bool,
    color: Color,
}

impl Tortue {
    pub fn new() -> Self {
        Tortue {
            position: Point::default(),
            angle: 0.0,
            points: vec![Point::default()],
            current: 0,
            pen_down: true,
            color: BLACK,
        }
    }

    pub fn arc_left(&mut self, _radius: Distance, _extent: Angle) {
        unimplemented!();
    }

    pub fn arc_right(&mut self, _radius: Distance, _extent: Angle) {
        unimplemented!();
    }

    pub fn backward(&mut self, _distance: Distance) {
        unimplemented!();
    }

    pub fn begin_fill(&mut self) {
        unimplemented!();
    }

    pub fn clear(&mut self) {
        unimplemented!();
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
            self.points.push(Point::new(new_x, new_y));
        }

        self.set_x(new_x);
        self.set_y(new_y);
    }

    pub fn go_to<P: Into<Point>>(&mut self, _position: P) {
        unimplemented!();
    }

    pub fn heading(&self) -> Angle {
        unimplemented!();
    }

    pub fn hide(&mut self) {
        unimplemented!();
    }

    pub fn home(&mut self) {
        unimplemented!();
    }

    pub fn is_filling(&self) -> bool {
        unimplemented!();
    }

    pub fn is_pen_down(&self) -> bool {
        unimplemented!();
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
        unimplemented!();
    }

    pub fn pen_down(&mut self) {
        self.pen_down = true;
    }

    pub fn pen_size(&self) -> f32 {
        unimplemented!();
    }

    pub fn pen_up(&mut self) {
        self.pen_down = false;
    }

    pub fn position(&self) -> Point {
        self.position
    }

    pub fn reset(&mut self) {
        unimplemented!();
    }
    pub fn right(&mut self, angle: Angle) {
        self.angle += angle;
    }

    pub fn set_fill_color(&mut self, _color: Color) {
        unimplemented!();
    }

    pub fn set_heading(&mut self, _angle: Angle) {
        unimplemented!();
    }

    pub fn set_pen_color(&mut self, _color: Color) {
        unimplemented!();
    }

    pub fn set_pen_size(&mut self, _thickness: f32) {
        unimplemented!();
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
    pub fn turn_towards<P: Into<Point>>(&mut self, _target: P) {
        unimplemented!();
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
                draw_line(pt1.x, pt1.y, pt2.x, pt2.y, 2.0, self.color);
            }

            // Update turtle position to the latest drawn point
            if ponts_to_draw > 0 {
                let new_pt = self.points[ponts_to_draw];
                self.set_x(new_pt.x);
                self.set_y(new_pt.y);
            }
        }

        // Draw turtle position
        draw_circle(self.position.x, self.position.y, 5.0, RED);

        // Advance drawing progress
        if self.current < self.points.len() - 1 {
            self.current += 1;
        }
    }
}
