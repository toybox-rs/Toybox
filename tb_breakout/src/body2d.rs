use super::vec2d::Vec2D;

/// A body is an object that has both position and velocity; e.g., a ball in Breakout.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Body2D {
    /// Where this object is located in two dimensions.
    pub position: Vec2D,
    /// How this object is moving in two dimensions.
    pub velocity: Vec2D,
}

impl Body2D {
    /// Create a new body from a position; no velocity.
    pub fn new_pos(x: f64, y: f64) -> Body2D {
        Body2D::new_detailed(x, y, 0.0, 0.0)
    }
    /// Create a body with both a position and a velocity.
    pub fn new_detailed(x: f64, y: f64, vx: f64, vy: f64) -> Body2D {
        Body2D {
            position: Vec2D::new(x, y),
            velocity: Vec2D::new(vx, vy),
        }
    }
    /// Update the position of this body based on a time step and its velocity.
    pub fn integrate_mut(&mut self, time_step: f64) {
        self.position += self.velocity.scale(time_step);
    }
}
