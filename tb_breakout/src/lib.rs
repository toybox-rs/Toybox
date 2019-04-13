//! The breakout crate contains the data structures and logic for a clone of the Atari 2600 game Breakout, but defined to be more flexible.

extern crate serde;
extern crate serde_json;
extern crate toybox_core;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate lazy_static;
extern crate ordered_float;
extern crate rand;

mod body2d;
/// This module contains the core logic of the game.
pub mod breakout;
mod font;
mod types;
mod vec2d;

pub use types::Breakout;
pub use types::State;
