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
        Input {
            left: false,
            right: false,
            up: false,
            down: false,
            button1: false,
            button2: false,
        }
    }
}
impl Input {
    pub fn new() -> Input {
        Input::default()
    }
    pub fn is_empty(self) -> bool {
        !self.left && !self.right && !self.up && !self.down && !self.button1 && !self.button2
    }
}

pub trait State {
    /// When true, this state should be replaced with a call to new_game() on the simulation.
    fn game_over(&self) -> bool;
    fn update_mut(&mut self, buttons: Input);
    fn draw(&self) -> Vec<graphics::Drawable>;
}

pub trait Simulation {
    fn new_game(&self) -> Box<State>;
    fn game_size(&self) -> (i32, i32);
}

pub fn get_simulation_by_name(name: &str) -> Result<Box<Simulation>, failure::Error> {
    let y: Result<Box<Simulation>, _> = match name.to_lowercase().as_str() {
        "amidar" => Ok(Box::new(amidar::Amidar)),
        "breakout" => Ok(Box::new(breakout::Breakout)),
        "space_invaders" => Ok(Box::new(space_invaders::SpaceInvaders)),
        "gridworld" => Ok(Box::new(gridworld::GridWorld::default())),
        _ => Err(format_err!(
            "Cannot construct game: `{}`. Try amidar, breakout, space_invaders.",
            name
        )),
    };
    y
}

pub const GAME_LIST: &[&str] = &["amidar", "breakout", "space_invaders", "gridworld"];

/// Amidar defined in this module.
pub mod amidar;
/// Breakout defined in this module.
pub mod breakout;
/// Space Invaders logic defined in this module.
pub mod space_invaders;
/// Gridworld
pub mod gridworld;