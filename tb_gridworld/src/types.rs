use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use toybox_core::graphics::Color;

// GridWorld enemies, to be simple, have a list of positions that they cycle through in order.
// They cause death.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Enemy {
    // List of (x,y) positions for this enemy.
    pub positions: Vec<(i32, i32)>,
    // Probably set this to zero by default. Which position should it start in?
    pub current_time: u32,
    // What color should it appear as?
    pub color: Color,
}

/// The tile behaviors in a GridWorld are configurable.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TileConfig {
    /// What reward (if any) is given or taken by passing this tile?
    pub reward: i32,
    /// Is this tile walkable by the agent?
    pub walkable: bool,
    /// Is this a terminal/goal tile?
    pub terminal: bool,
    /// What color should this tile be?
    pub color: Color,
}

/// The initial game state is configurable.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GridWorld {
    /// How is the board laid out? List of rows.
    pub grid: Vec<String>,
    /// How are tiles defined?
    pub tiles: HashMap<char, TileConfig>,
    /// When you take a reward, it must change tile type.
    pub reward_becomes: char,
    /// What color is the player?
    pub player_color: Color,
    /// Where does the player start?
    pub player_start: (i32, i32),
    /// Does this world support diagonal movement?
    pub diagonal_support: bool,
    /// Does this world have enemies?
    pub enemies: Vec<Enemy>,
}

/// The game state is composed of a configuration and the current frame.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct State {
    /// The initial game state.
    pub config: GridWorld,
    /// The current frame state.
    pub frame: FrameState,
}

/// This represents the mutable state of the gridworld system.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FrameState {
    /// True if you have encountered a terminal tile.
    pub game_over: bool,
    /// How much reward has been earned (cumulative).
    pub score: i32,
    /// How many steps has this simulation run for?
    pub step: usize,
    /// When you take a reward, it must change tile type.
    pub reward_becomes: char,
    /// How are tiles defined?
    pub tiles: HashMap<char, TileConfig>,
    /// How is the board laid out? List of rows.
    pub grid: Vec<String>,
    /// The player position.
    pub player: (i32, i32),
    /// Does this world have enemies? Where have they moved to?
    pub enemies: Vec<Enemy>,
}

/// Enumeration that supports diagonal movement.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DiagonalDir {
    NE,
    N,
    NW,
    E,
    W,
    SE,
    S,
    SW,
}
