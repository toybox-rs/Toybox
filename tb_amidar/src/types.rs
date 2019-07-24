use serde::{Deserialize, Serialize};
use toybox_core::graphics::Color;
use toybox_core::random;
use toybox_core::Direction;

use std::collections::{HashSet, VecDeque};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Amidar {
    pub rand: random::Gen,
    pub board: Vec<String>,
    pub player_start: TilePoint,
    pub bg_color: Color,
    pub player_color: Color,
    pub unpainted_color: Color,
    pub painted_color: Color,
    pub enemy_color: Color,
    pub inner_painted_color: Color,
    pub start_lives: i32,
    pub start_jumps: i32,
    pub render_images: bool,
    pub chase_time: i32,
    pub chase_score_bonus: i32,
    pub jump_time: i32,
    pub box_bonus: i32,
    /// This should be false if you ever use a non-default board.
    pub default_board_bugs: bool,
    pub enemies: Vec<MovementAI>,
    pub level: i32,
    /// How many previous junctions should the player and enemies remember?
    pub history_limit: u32,
    pub enemy_starting_speed: i32,
    pub player_speed: i32,
}

#[derive(Debug, Clone)]
pub struct ScreenPoint {
    pub sx: i32,
    pub sy: i32,
}

/// Strongly-typed vector for "world" positioning in Amidar.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorldPoint {
    pub x: i32,
    pub y: i32,
}

/// Strongly-typed vector for "tile" positioning in Amidar.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TilePoint {
    pub tx: i32,
    pub ty: i32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GridBox {
    pub top_left: TilePoint,
    pub bottom_right: TilePoint,
    pub painted: bool,
    pub triggers_chase: bool,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub enum Tile {
    Empty,
    Unpainted,
    ChaseMarker,
    Painted,
}

#[derive(Clone, PartialEq, Serialize, Deserialize, Debug)]
pub enum MovementAI {
    Player,
    EnemyLookupAI {
        next: u32,
        default_route_index: u32,
    },
    EnemyPerimeterAI {
        start: TilePoint,
    },
    EnemyAmidarMvmt {
        vert: Direction,
        horiz: Direction,
        start_vert: Direction,
        start_horiz: Direction,
        start: TilePoint,
    },
    EnemyRandomMvmt {
        start: TilePoint,
        start_dir: Direction,
        dir: Direction,
    },
    EnemyTargetPlayer {
        start: TilePoint,
        start_dir: Direction,
        vision_distance: i32,
        dir: Direction,
        player_seen: Option<TilePoint>,
    },
}

/// Mob is a videogame slang for "mobile" unit. Players and Enemies are the same struct.
#[derive(Clone, Serialize, Deserialize)]
pub struct Mob {
    pub ai: MovementAI,
    pub position: WorldPoint,
    pub caught: bool,
    pub speed: i32,
    pub step: Option<TilePoint>,
    pub history: VecDeque<u32>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Board {
    pub tiles: Vec<Vec<Tile>>,
    pub width: u32,
    pub height: u32,
    pub junctions: HashSet<u32>,
    pub chase_junctions: HashSet<u32>,
    pub boxes: Vec<GridBox>,
}

#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub struct BoardUpdate {
    pub vertical: i32,
    pub horizontal: i32,
    pub num_boxes: i32,
    pub triggers_chase: bool,
    pub junctions: Option<(u32, u32)>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct StateCore {
    pub rand: random::Gen,
    pub score: i32,
    pub lives: i32,
    pub jumps: i32,
    pub chase_timer: i32,
    pub jump_timer: i32,
    pub player: Mob,
    pub enemies: Vec<Mob>,
    pub board: Board,
    pub level: i32,
}

pub struct State {
    pub config: Amidar,
    pub state: StateCore,
}

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum EnemyPlayerState {
    Miss,
    PlayerDeath,
    EnemyCatch(usize),
}
