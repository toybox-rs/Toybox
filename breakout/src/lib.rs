extern crate failure;
extern crate serde;
extern crate serde_json;
extern crate toybox_core;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate lazy_static;

mod body2d;
mod breakout;
mod font;
mod vec2d;

pub use breakout::screen;
pub use breakout::Breakout;
pub use breakout::State;
