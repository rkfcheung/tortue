use macroquad::prelude::*;
use std::f32::consts::PI;

use crate::prelude::*;

const DEGREES_TO_RADIANS: f32 = PI / 180.0;
const PI_DIV_2: f32 = PI / 2.0;

pub struct Tortue {
    position: Point,
    angle: Angle,
    points: Vec<Point>,
    current: usize,
    color: Color,
    pen_color: Color,
    pen_down: bool,
    pen_size: f32,
    filling: bool,
    fill_color: Color,
    visible: bool,
    use_degrees: bool,
    speed: Speed,
}

impl Tortue {
    pub fn new() -> Self {
        Self {
            position: origin(),
            angle: 0.0,
            points: vec![origin()],
            current: 0,
            color: GREEN,
            pen_color: BLACK,
            pen_down: true,
            pen_size: 2.0,
            filling: false,
            fill_color: GRAY,
            visible: true,
            use_degrees: true,
            speed: 1,
        }
    }

    pub fn arc_left(&mut self, radius: Distance, extent: Angle) {
        let step = if self.use_degrees {
            1.0
        } else {
            DEGREES_TO_RADIANS
        };
        let num_steps = (extent / step) as usize;

        for _ in 0..num_steps {
            let rad = self.angle.to_radians();
            let step_x = radius * (rad + PI_DIV_2).cos();
            let step_y = radius * (rad + PI_DIV_2).sin();

            if self.pen_down {
                self.points
                    .push((self.position.x + step_x, self.position.y + step_y).into());
            }

            self.left(step);
        }
    }

    pub fn arc_right(&mut self, radius: Distance, extent: Angle) {
        let step = if self.use_degrees {
            1.0
        } else {
            DEGREES_TO_RADIANS
        };
        let num_steps = (extent / step) as usize;

        for _ in 0..num_steps {
            let rad = self.angle.to_radians();
            let step_x = radius * (rad - PI_DIV_2).cos();
            let step_y = radius * (rad - PI_DIV_2).sin();

            if self.pen_down {
                self.points
                    .push((self.position.x + step_x, self.position.y + step_y).into());
            }

            self.right(step);
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
        self.filling = true;
    }

    pub fn clear(&mut self) {
        // Reset to initial state
        self.position = origin();
        self.angle = 0.0;
        self.points = vec![origin()];
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
        self.filling = false;
        if self.points.len() >= 3 {
            let vertices: Vec<Vec2> = self.points.iter().map(|p| vec2(p.x, p.y)).collect();
            for i in 1..vertices.len() - 1 {
                draw_triangle(
                    vertices[0],     // First point as base
                    vertices[i],     // Next point
                    vertices[i + 1], // Next-next point
                    self.fill_color, // Fill colour
                );
            }
        }
    }

    pub fn fill_color(&self) -> Color {
        self.fill_color
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
        self.visible = false;
    }

    pub fn home(&mut self) {
        self.go_to(origin());
        self.angle = 0.0;
    }

    pub fn is_filling(&self) -> bool {
        self.filling
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
        self.visible
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

    pub fn set_color(&mut self, color: Color) {
        self.color = color;
    }

    pub fn set_fill_color(&mut self, color: Color) {
        self.fill_color = color;
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

    pub fn set_speed(&mut self, speed: Speed) {
        self.speed = speed;
    }

    pub fn set_x(&mut self, x: f32) {
        self.position.x = x;
    }

    pub fn set_y(&mut self, y: f32) {
        self.position.y = y;
    }

    pub fn show(&mut self) {
        self.visible = true;
    }

    pub fn speed(&self) -> Speed {
        self.speed
    }

    pub fn turn_towards<P: Into<Point>>(&mut self, target: P) {
        let target_point = target.into();
        let dx = target_point.x - self.position.x;
        let dy = target_point.y - self.position.y;

        let target_angle = dy.atan2(dx).to_degrees();
        self.set_heading(target_angle);
    }

    pub fn update(&mut self) {
        clear_background(WHITE);

        let current = self.current;

        if self.points.len() > 1 {
            for i in 0..current {
                let pt1 = self.points[i];
                let pt2 = self.points[i + 1];
                draw_line(pt1.x, pt1.y, pt2.x, pt2.y, self.pen_size, self.pen_color);
            }

            if current > 0 {
                let new_pt = self.points[current];
                self.set_x(new_pt.x);
                self.set_y(new_pt.y);
            }
        }

        if self.visible {
            draw_circle(self.position.x, self.position.y, 5.0, self.color);
        }

        if self.current < self.points.len() - 1 {
            self.current += 1;
        }
    }

    pub fn use_degrees(&mut self) {
        self.use_degrees = true;
    }

    pub fn use_radians(&mut self) {
        self.use_degrees = false;
    }

    pub fn wait_for_click(&mut self) {
        unimplemented!();
    }

    pub fn wait(&self, secs: f64) {
        let start_time = get_time();
        while get_time() - start_time < secs {}
    }
}
