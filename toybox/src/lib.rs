#[macro_use]
extern crate failure;

#[macro_use]
extern crate lazy_static;

pub mod graphics;

use std::f64;

mod direction;
pub use direction::Direction;

mod vec2d;
pub use vec2d::Vec2D;

#[derive(Debug, Clone)]
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

/// Think NES-style controls: directions, and two buttons.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Input {
    pub left: bool,
    pub right: bool,
    pub up: bool,
    pub down: bool,
    pub button1: bool,
    pub button2: bool,
}

impl Default for Input {
    fn default() -> Self {
        Input { left: false, right: false, up: false, down: false, button1: false, button2: false }
    }
}
impl Input {
    pub fn new() -> Input {
        Input::default()
    }
    pub fn is_empty(&self) -> bool {
        return !self.left && !self.right && !self.up && !self.down && !self.button1 && !self.button2;
    }
}

/// Amidar defined in this module.
pub mod amidar;
/// Breakout defined in this module.
pub mod breakout;
/// Space Invaders logic defined in this module.
pub mod space_invaders;