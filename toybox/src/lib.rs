#[macro_use]
extern crate failure;

#[macro_use]
extern crate lazy_static;

extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

extern crate png;
extern crate toybox_core;

pub mod digit_sprites;

mod direction;
/// Direction represents an enum of Left,Right,Up and Down.
pub use direction::Direction;

mod vec2d;
/// Vec2D represents a floating (x,y) coordinate or vector.
pub use vec2d::Vec2D;

mod body2d;
/// Body2D represents an object with position, velocity and acceleration in 2D.
pub use body2d::Body2D;

pub use toybox_core::graphics;
pub use toybox_core::random;
/// Input represents the buttons pressed given to our games.
pub use toybox_core::Input;
pub use toybox_core::Simulation;
pub use toybox_core::State;

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
