/// This represents a point or a size or a velocity in 2 dimensions.
/// We use f64 for internal representations but we can get integer coordinates upon request for drawing.
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Vec2D {
    /// The x-coordinate of this vector.
    pub x: f64,
    /// The y-coordinate of this vector.
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

    /// The magnitude of the vector.
    pub fn magnitude(&self) -> f64 {
        (self.x * self.x + self.y * self.y).sqrt()
    }

    /// The squared magnitude of this vector; cheaper than magnitude if you just want to know which vector is biggest.
    pub fn magnitude_squared(&self) -> f64 {
        (self.x * self.x + self.y * self.y)
    }

    /// Get the angle of this vector in radians.
    pub fn angle(&self) -> f64 {
        self.y.atan2(self.x)
    }

    /// Move a vector by another and produce a new vector.
    pub fn translate(&self, by: &Vec2D) -> Vec2D {
        Vec2D::new(self.x + by.x, self.y + by.y)
    }

    /// Move a vector by another; modifying it.
    pub fn translate_mut(&mut self, by: &Vec2D) {
        self.x += by.x;
        self.y += by.y;
    }

    /// Scale a vector by a constant, producing a new vector.
    pub fn scale(&self, by: f64) -> Vec2D {
        Vec2D::new(self.x * by, self.y * by)
    }

    /// Scale a vector by a constant, modifying it.
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

/// Support using the plus operator on vectors.
impl Add for Vec2D {
    type Output = Vec2D;

    /// This defers to translate; producing a new vector from the sum of two others.
    fn add(self, other: Vec2D) -> Vec2D {
        self.translate(&other)
    }
}

/// Support using the += operator on vectors.
impl AddAssign for Vec2D {
    /// This defers to translate_mut; translating a vector by another.
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
