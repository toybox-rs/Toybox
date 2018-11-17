#[macro_use]
extern crate failure;

#[macro_use]
extern crate lazy_static;

extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

extern crate png;

use std::any::Any;

pub mod digit_sprites;
pub mod graphics;

mod random;

mod direction;
/// Direction represents an enum of Left,Right,Up and Down.
pub use direction::Direction;

mod vec2d;
/// Vec2D represents a floating (x,y) coordinate or vector.
pub use vec2d::Vec2D;

mod body2d;
/// Body2D represents an object with position, velocity and acceleration in 2D.
pub use body2d::Body2D;

mod input;
/// Input represents the buttons pressed given to our games.
pub use input::Input;

/// This trait models a single frame state for a Simulation.
pub trait State {
    /// For dynamic casts.
    fn as_any(&self) -> &Any;
    /// When < 0, this state should be replaced with a call to new_game() on the simulation.
    fn lives(&self) -> i32;
    /// Get the score from the game, i32 allows for negative scores.
    fn score(&self) -> i32;
    /// To update internally to the next state, we pass buttons to internal logic.
    fn update_mut(&mut self, buttons: Input);
    /// Any state can create a vector of drawable objects to present itself.
    fn draw(&self) -> Vec<graphics::Drawable>;
    /// Any state can serialize to JSON String.
    fn to_json(&self) -> String;
    /// Separate serialization of once-per-game data
    fn config_to_json(&self) -> String;
}

/// This trait models a simulation or game. It knows how to start a new game, and to declare its size before any gameplay starts.
pub trait Simulation {
    /// For dynamic casts.
    fn as_any(&self) -> &Any;
    /// Seed simulation.
    fn reset_seed(&mut self, seed: u32);
    /// Generate a new State. This is in a Box<State> because it may be 1 of many unknown types as far as calling code is concerned.
    fn new_game(&mut self) -> Box<State>;
    /// Return a tuple of game size in pixels, e.g., (100,100).
    fn game_size(&self) -> (i32, i32);
    /// Generate a new state from JSON String. Uses the default config.
    fn new_state_from_json(&self, json: &str) -> Result<Box<State>, failure::Error>;
    /// Generate new state and new config from JSON String.
    fn new_state_config_from_json(
        &self,
        json_config: &str,
        json_state: &str,
    ) -> Result<Box<State>, failure::Error>;
}

/// This method returns a Box<Simulation> if possible for a given game name.
pub fn get_simulation_by_name(name: &str) -> Result<Box<Simulation>, failure::Error> {
    let y: Result<Box<Simulation>, _> = match name.to_lowercase().as_str() {
        #[cfg(feature = "amidar")]
        "amidar" => Ok(Box::new(amidar::Amidar)),
        #[cfg(feature = "breakout")]
        "breakout" => Ok(Box::new(breakout::Breakout::default())),
        #[cfg(feature = "space_invaders")]
        "space_invaders" => Ok(Box::new(space_invaders::SpaceInvaders)),
        #[cfg(feature = "gridworld")]
        "gridworld" => Ok(Box::new(gridworld::GridWorld::default())),
        _ => Err(format_err!(
            "Cannot construct game: `{}`. Try any of {:?}.",
            name,
            GAME_LIST
        )),
    };
    y
}

/// This defines the set of games that are known. An index into this array is used in human_play, so try not to shuffle them!
pub const GAME_LIST: &[&str] = &[
    #[cfg(feature = "amidar")]
    "amidar",
    #[cfg(feature = "breakout")]
    "breakout",
    #[cfg(feature = "space_invaders")]
    "space_invaders",
    #[cfg(feature = "gridworld")]
    "gridworld",
];

/// Amidar defined in this module.
#[cfg(feature = "amidar")]
pub mod amidar;
/// Breakout defined in this module.
#[cfg(feature = "breakout")]
pub mod breakout;
/// Gridworld
#[cfg(feature = "gridworld")]
pub mod gridworld;
/// Queries
pub mod queries;
/// Space Invaders logic defined in this module.
#[cfg(feature = "space_invaders")]
pub mod space_invaders;
