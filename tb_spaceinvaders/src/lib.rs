#[macro_use]
extern crate failure;
extern crate itertools;
extern crate serde;
extern crate serde_json;
extern crate toybox_core;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate lazy_static;

mod destruction;
mod font;
mod space_invaders;

pub use space_invaders::screen;
pub use space_invaders::SpaceInvaders;
pub use space_invaders::State;
