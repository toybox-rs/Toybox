extern crate serde;
extern crate serde_json;
extern crate toybox_core;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate failure;

pub mod amidar;
mod digit_sprites;

pub use amidar::Amidar;
pub use amidar::State;