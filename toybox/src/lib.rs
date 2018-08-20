use std::f64;

/// This represents a point or a size or a velocity in 2 dimensions.
/// We use f64 for internal representations but we can get integer coordinates upon request for drawing.
#[derive(Debug,PartialEq)]
pub struct Vec2D {
    x: f64,
    y: f64,
}

impl Vec2D {
    /// Create a new vector at the origin.
    pub fn origin() -> Vec2D {
        Vec2D { x: 0.0, y: 0.0 }
    }
    /// Create a new vector from components.
    pub fn new(x: f64, y: f64) -> Vec2D {
        Vec2D { x, y }
    }
    /// Create a new vector from angle and speed.
    pub fn from_polar(r: f64, theta: f64) -> Vec2D {
        Vec2D::new(r * theta.cos(), r * theta.sin())
    }

    pub fn magnitude(&self) -> f64 {
        (self.x*self.x + self.y*self.y).sqrt()
    }

    /// Get the angle of this vector in radians.
    pub fn angle(&self) -> f64 {
        self.y.atan2(self.x)
    }

    pub fn translate(&self, by: &Vec2D) -> Vec2D {
        Vec2D::new(self.x + by.x, self.y + by.y)
    }
    /// In order to render, we want pixel coordinates.
    pub fn pixels(&self) -> (i32, i32) {
        (self.x.floor() as i32, self.y.floor() as i32)
    }
}

pub struct Body2D {
    pos: Vec2D,
    vel: Vec2D,
    accel: Vec2D,
}

enum Drawable {
    Rectangle(u32, u32, u32, u32),
    Circle(u32, u32, u32),
}

trait GameVisualOutput {
    fn game_size() -> Vec2D;
    fn display_list() -> Vec<Drawable>;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut ball = Vec2D::new(7.0,3.0);
        let velocity = Vec2D::new(4.0,4.0);
        let pos = ball.translate(&velocity);
        assert_eq!(pos, Vec2D::new(11.0,7.0));
        assert_eq!(velocity.magnitude(), (32.0 as f64).sqrt())
    }
}
