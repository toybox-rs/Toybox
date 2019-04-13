//! The breakout crate contains the data structures and logic for a clone of the Atari 2600 game Breakout, but defined to be more flexible.
//!
//! None of the modules in this crate are public. The `Breakout` struct is the `toybox_core::Simulation` and the `State` struct is the `toybox_core::State` used generically by other crates.

extern crate serde;
extern crate serde_json;
extern crate toybox_core;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate lazy_static;
extern crate ordered_float;
extern crate rand;

/// This module contains a 2d body (position, velocity) used in Breakout.
mod body2d;
/// This module contains a 2d vector used in Breakout.
mod vec2d;

/// This module contains the core logic of the game.
mod breakout;
/// This module contains the font used for rendering scores.
mod font;
/// This module contains the core data structures used in the game.
mod types;

pub use body2d::Body2D;
pub use types::{Breakout, Brick, StartBall, State, StateCore};
pub use vec2d::Vec2D;
