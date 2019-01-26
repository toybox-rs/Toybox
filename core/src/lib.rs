extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
extern crate png;

pub mod collision;
pub mod graphics;
pub mod random;

mod input;
pub use input::AleAction;
pub use input::Input;

mod direction;
pub use direction::Direction;

extern crate rand_core;

/// This enum defines failure conditions for a query_json call.
#[derive(PartialEq, Eq, Hash, PartialOrd, Ord, Debug)]
pub enum QueryError {
    NoSuchQuery,
    BadInputArg,
    InternalSerializationError(String),
}

impl From<serde_json::Error> for QueryError {
    fn from(e: serde_json::Error) -> QueryError {
        QueryError::InternalSerializationError(format!("{}", e))
    }
}

/// This trait models a single frame state for a Simulation.
pub trait State {
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
    /// Submit a query to this state object, returning a JSON String or error message.
    fn query_json(&self, query: &str, args: &serde_json::Value) -> Result<String, QueryError>;
}

/// This trait models a simulation or game. It knows how to start a new game, and to declare its size before any gameplay starts.
pub trait Simulation {
    /// Seed simulation.
    fn reset_seed(&mut self, seed: u32);

    /// Generate a new State. This is in a Box<State> because it may be 1 of many unknown types as far as calling code is concerned.
    fn new_game(&mut self) -> Box<State>;
    /// Generate a new State from JSON String (usually modified from a dump of State::to_json).
    fn new_state_from_json(&self, json: &str) -> Result<Box<State>, serde_json::Error>;

    /// Return a tuple of game size in pixels, e.g., (100,100).
    fn game_size(&self) -> (i32, i32);

    /// This serializes the "config" for a game to json.
    fn to_json(&self) -> String;
    /// This deserializes the "config" for a game from json.
    /// Generate new state and new config from JSON String.
    fn from_json(&self, json: &str) -> Result<Box<Simulation>, serde_json::Error>;

    /// Legal action set:
    fn legal_action_set(&self) -> Vec<AleAction>;
}
