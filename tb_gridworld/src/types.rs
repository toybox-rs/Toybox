use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use toybox_core::graphics::Color;

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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GridWorld {
    pub grid: Vec<String>,
    pub tiles: HashMap<char, TileConfig>,
    pub reward_becomes: char,
    pub player_color: Color,
    pub player_start: (i32, i32),
    /// Does this world support diagonal movement?
    pub diagonal_support: bool,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct State {
    pub config: GridWorld,
    pub frame: FrameState,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FrameState {
    pub game_over: bool,
    pub score: i32,
    pub step: usize,
    pub reward_becomes: usize,
    pub tiles: Vec<TileConfig>,
    pub grid: Vec<Vec<usize>>,
    pub player: (i32, i32),
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
