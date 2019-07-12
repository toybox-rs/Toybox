extern crate itertools;
extern crate serde;
extern crate serde_json;
extern crate toybox_core;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate lazy_static;
extern crate rand;

mod destruction;
mod firing_ai;
mod font;
mod spaceinvaders;
mod types;

// All types are essentially "public" API.
pub use firing_ai::FiringAI;
pub use types::Enemy;
pub use types::Laser;
pub use types::Player;
pub use types::SpaceInvaders;
pub use types::State;
pub use types::StateCore;
pub use types::Ufo;
