extern crate failure;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
extern crate png;

pub mod graphics;
pub mod random;

mod input;
pub use input::Input;

mod direction;
pub use direction::Direction;

use std::any::Any;

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
