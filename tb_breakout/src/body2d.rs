use super::vec2d::Vec2D;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Body2D {
    pub position: Vec2D,
    pub velocity: Vec2D,
    pub acceleration: Vec2D,
}

impl Body2D {
    pub fn new_pos(x: f64, y: f64) -> Body2D {
        Body2D::new_detailed(x, y, 0.0, 0.0, 0.0, 0.0)
    }
    pub fn new_detailed(x: f64, y: f64, vx: f64, vy: f64, ax: f64, ay: f64) -> Body2D {
        Body2D {
            position: Vec2D::new(x, y),
            velocity: Vec2D::new(vx, vy),
            acceleration: Vec2D::new(ax, ay),
        }
    }
    pub fn integrate_mut(&mut self, time_step: f64) {
        self.position += self.velocity.scale(time_step);
        self.velocity += self.acceleration.scale(time_step);
    }
}
