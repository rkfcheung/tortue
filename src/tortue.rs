// Ported from https://github.com/sunjay/turtle/blob/master/src/turtle.rs

use macroquad::prelude::*;
use std::f32::consts::PI;
use std::thread::sleep;
use std::time::Duration;

use crate::prelude::*;

// Conversion constants
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
        Self::new_with(origin())
    }

    pub fn new_with(position: Point) -> Self {
        Self {
            position,
            angle: 0.0,
            points: vec![position],
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

    /// Move the tortue backwards by the given amount of `distance`. If the pen is down, the tortue
    /// will draw a line as it moves.
    ///
    /// The tortue takes very small steps (measured in "pixels"). So if you want it to move more,
    /// use a bigger value to make the tortue walk further.
    /// The `distance` can be a negative value, in which case the tortue will move forward.
    /// If `distance` is NaN or infinite, the tortue does not move.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use tortue::prelude::*;
    ///
    /// let origin = Point::new(0.0, 0.0);
    /// let mut tortue = Tortue::new_with(origin);
    /// // Initially at (0.0, 0.0), facing east (0 degrees)
    /// assert_eq!(tortue.position(), origin);
    ///
    /// // Move backward 10 units; draws a line since pen is down by default
    /// tortue.backward(10.0);
    /// assert_eq!(tortue.position(), Point::new(-10.0, 0.0));
    ///
    /// // Move backward 100 more units, continuing to draw
    /// tortue.backward(100.0);
    /// assert_eq!(tortue.position(), Point::new(-110.0, 0.0));
    ///
    /// // Lift pen and move forward 223 units using negative distance
    /// tortue.pen_up();
    /// tortue.backward(-223.0); // Negative distance moves forward
    /// assert_eq!(tortue.position(), Point::new(113.0, 0.0));
    ///
    /// // Turn 90 degrees (north), lower pen, and move backward 50 units
    /// tortue.set_heading(90.0);
    /// tortue.pen_down();
    /// tortue.backward(50.0);
    /// assert_eq!(tortue.position(), Point::new(113.0, -50.0));
    ///
    /// // Move forward 50 units using negative distance, drawing a line
    /// tortue.backward(-50.0);
    /// assert_eq!(tortue.position(), Point::new(113.0, 0.0));
    ///
    /// // Zero distance does nothing
    /// tortue.backward(0.0);
    /// assert_eq!(tortue.position(), Point::new(113.0, 0.0));
    ///
    /// // NaN distance does nothing
    /// tortue.backward(f32::NAN);
    /// assert_eq!(tortue.position(), Point::new(113.0, 0.0));
    ///
    /// // Infinite distance does nothing
    /// tortue.backward(f32::INFINITY);
    /// assert_eq!(tortue.position(), Point::new(113.0, 0.0));
    /// ```
    pub fn backward(&mut self, distance: Distance) {
        self.move_by(distance, -1.0);
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

    /// Move the tortue forward by the given amount of `distance`. If the pen is down, the tortue
    /// will draw a line as it moves.
    ///
    /// The tortue takes very small steps (measured in "pixels"). So if you want it to move more,
    /// use a bigger value to make the tortue walk further.
    /// The `distance` can be a negative value, in which case the tortue will move backward.
    /// If `distance` is NaN or infinite, the tortue does not move.
    ///
    /// # Examples
    ///
    /// ```
    /// use tortue::prelude::*;
    ///
    /// let mut tortue = Tortue::new_with((0.0, 0.0).into());
    /// // Initially at (0.0, 0.0), facing east (0 degrees)
    /// assert_eq!(tortue.position(), (0.0, 0.0).into());
    ///
    /// // Move forward 10 units, drawing if pen is down (default is typically down)
    /// tortue.forward(10.0);
    /// assert_eq!(tortue.position(), (10.0, 0.0).into());
    ///
    /// // Move forward 100 more units
    /// tortue.forward(100.0);
    /// assert_eq!(tortue.position(), (110.0, 0.0).into());
    ///
    /// // Lift pen and move backward 223 units
    /// tortue.pen_up();
    /// tortue.forward(-223.0);
    /// assert_eq!(tortue.position(), (-113.0, 0.0).into());
    ///
    /// // Turn 90 degrees (north) and move forward 50 units
    /// tortue.set_heading(90.0);
    /// tortue.pen_down();
    /// tortue.forward(50.0);
    /// assert_eq!(tortue.position(), (-113.0, 50.0).into());
    ///
    /// // Move backward 50 units (south)
    /// tortue.forward(-50.0);
    /// assert_eq!(tortue.position(), (-113.0, 0.0).into());
    ///
    /// // Zero distance does nothing
    /// tortue.forward(0.0);
    /// assert_eq!(tortue.position(), (-113.0, 0.0).into());
    ///
    /// // NaN distance does nothing
    /// tortue.forward(f32::NAN);
    /// assert_eq!(tortue.position(), (-113.0, 0.0).into());
    ///
    /// // Infinite distance does nothing
    /// tortue.forward(f32::INFINITY);
    /// assert_eq!(tortue.position(), (-113.0, 0.0).into());
    /// ```
    pub fn forward(&mut self, distance: Distance) {
        self.move_by(distance, 1.0);
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
        self.use_degrees
    }

    pub fn is_using_radians(&self) -> bool {
        !self.use_degrees
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

        // Compute target angle in degrees (using atan2 returns radians, so convert if needed)
        let target_angle_deg = dy.atan2(dx).to_degrees();
        self.set_heading(target_angle_deg);
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
            if current < self.points.len() {
                self.position = self.points[current];
            }
        }

        if self.visible {
            draw_circle(self.position.x, self.position.y, 5.0, self.color);
        }

        if self.current < self.points.len() - 1 {
            self.current += self.speed;
        }
    }

    pub fn use_degrees(&mut self) {
        self.use_degrees = true;
    }

    pub fn use_radians(&mut self) {
        self.use_degrees = false;
    }

    /// Wait for a mouse click before continuing.
    ///
    /// This implementation uses macroquad's input functions and busy-waits (with a short sleep)
    /// so that it does not peg the CPU.
    pub fn wait_for_click(&mut self) {
        // Loop until the left mouse button is pressed.
        while !is_mouse_button_pressed(MouseButton::Left) {
            sleep(Duration::from_millis(16)); // roughly 60 FPS
        }
    }

    /// Wait for the specified number of seconds.
    pub fn wait(&self, secs: f64) {
        let start_time = get_time();
        while get_time() - start_time < secs {}
    }

    // Internal helper for moving forward or backward.
    fn move_by(&mut self, distance: Distance, multiplier: f32) {
        if !distance.is_finite() {
            return;
        }
        let rad = self.angle.to_radians();
        let new_x = self.position.x + multiplier * distance * rad.cos();
        let new_y = self.position.y + multiplier * distance * rad.sin();

        if self.pen_down {
            self.points.push((new_x, new_y).into());
        }
        self.set_x(new_x);
        self.set_y(new_y);
    }
}
