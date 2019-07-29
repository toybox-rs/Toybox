use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use toybox_core::graphics::Color;
use toybox_core::random;

/// GridWorld enemies, to be simple, have a list of positions that they cycle through in order.
/// They cause death.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Enemy {
    /// List of (x,y) positions for this enemy.
    pub positions: Vec<(i32, i32)>,
    /// Probably set this to zero by default. Which position should it start in?
    pub current_time: u32,
    /// What color should it appear as?
    pub color: Color,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TileBehavior {
    /// A character here is the tile-identifier of the walkability to toggle.
    /// The first color
    DoorSwitch {
        switch_id: u32,
        state: bool,
        on_color: Color,
        off_color: Color,
    },
    /// Door: a maybe-walkable tile, connected to a switch.
    Door {
        switch_id: u32,
        open: Color,
        closed: Color,
    },
    /// Receive One-Time Reward: becomes floor after.
    ReceiveReward(i32),
    /// Lose by stepping here.
    LoseGame,
    /// Win by stepping here.
    WinGame,
    /// Lose game with some probability.
    MaybeLoseGame(f64, Color),
    /// Not-walkable.
    Wall,
    /// Walkable tile.
    Floor,
}

/// The initial game state is configurable.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GridWorld {
    /// How is the board laid out? List of rows.
    pub grid: Vec<String>,
    /// How are tiles defined?
    pub tiles: HashMap<char, TileBehavior>,
    /// Where does the player start?
    pub player_start: (i32, i32),
    /// Does this world support diagonal movement?
    pub diagonal_support: bool,
    /// Does this world have enemies?
    pub enemies: Vec<Enemy>,
    /// Random number generator used to seed new games.
    pub rand: random::Gen,
    /// Reward Difference on Lose:
    pub lose_reward: i32,
    /// Reward Difference on Win:
    pub win_reward: i32,

    /// What color is the player?
    pub player_color: Color,
    /// Win tile color:
    pub win_color: Color,
    /// Lose tile color:
    pub lose_color: Color,
    /// Wall tile color:
    pub wall_color: Color,
    /// Floor tile color:
    pub floor_color: Color,
    /// One-time reward color:
    pub reward_color: Color,
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
    /// How is the board laid out? List of rows.
    pub grid: Vec<Vec<TileBehavior>>,
    /// The player position.
    pub player: (i32, i32),
    /// Does this world have enemies? Where have they moved to?
    pub enemies: Vec<Enemy>,
    /// Random number generator used to determine probabilistic tile terminals.
    pub rand: random::Gen,
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
