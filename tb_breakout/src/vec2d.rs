/// This represents a point or a size or a velocity in 2 dimensions.
/// We use f64 for internal representations but we can get integer coordinates upon request for drawing.
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Vec2D {
    pub x: f64,
    pub y: f64,
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
        (self.x * self.x + self.y * self.y).sqrt()
    }
    pub fn magnitude_squared(&self) -> f64 {
        (self.x * self.x + self.y * self.y)
    }

    /// Get the angle of this vector in radians.
    pub fn angle(&self) -> f64 {
        self.y.atan2(self.x)
    }

    pub fn translate(&self, by: &Vec2D) -> Vec2D {
        Vec2D::new(self.x + by.x, self.y + by.y)
    }
    pub fn translate_mut(&mut self, by: &Vec2D) {
        self.x += by.x;
        self.y += by.y;
    }
    pub fn scale(&self, by: f64) -> Vec2D {
        Vec2D::new(self.x * by, self.y * by)
    }
    pub fn scale_mut(&mut self, by: f64) {
        self.x *= by;
        self.y *= by;
    }

    /// In order to render, we want pixel coordinates.
    pub fn pixels(&self) -> (i32, i32) {
        (self.x.floor() as i32, self.y.floor() as i32)
    }
}

// For operator overloading.
use std::ops::{Add, AddAssign};

impl Add for Vec2D {
    type Output = Vec2D;
    fn add(self, other: Vec2D) -> Vec2D {
        self.translate(&other)
    }
}
impl AddAssign for Vec2D {
    fn add_assign(&mut self, other: Vec2D) {
        self.translate_mut(&other)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let ball = Vec2D::new(7.0, 3.0);
        let velocity = Vec2D::new(4.0, 4.0);
        let pos = ball.translate(&velocity);
        assert_eq!(pos, Vec2D::new(11.0, 7.0));
        assert_eq!(velocity.magnitude(), (32.0 as f64).sqrt())
    }
}
