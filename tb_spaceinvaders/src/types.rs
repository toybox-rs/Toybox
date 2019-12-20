use firing_ai::FiringAI;
use serde::{Deserialize, Serialize};
use toybox_core::graphics::{Color, SpriteData};
use toybox_core::random;
use toybox_core::Direction;

/// The player's ship is represented by this structure.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Player {
    /// The x-coordinate of the player; this is controllable.
    pub x: i32,
    /// The y-coordinate of the player; no keys affect this.
    pub y: i32,
    /// The width of the player ship.
    pub w: i32,
    /// The hight of the player ship.
    pub h: i32,
    /// Speed of movement of the player on a key-press.
    pub speed: i32,
    /// The color of the player ship.
    pub color: Color,
    /// Whether or not the player is alive.
    pub alive: bool,
    /// This is an animation counter; the presence of a value here means that the player is in the process of dying.
    pub death_counter: Option<i32>,
    /// This is an animation flag; it is set based on the value of death_counter.
    pub death_hit_1: bool,
}

/// Each shot in SpaceInvaders by the player or the enemy is a Laser object.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Laser {
    /// The x-coordinate of the laser.
    pub x: i32,
    /// The y-coordinate of the laser.
    pub y: i32,
    /// The width of the laser; the laser itself is a rectangle.
    pub w: i32,
    /// The height of the laser; the laser itself is a rectangle.
    pub h: i32,
    /// Laser timing (visible / not-visible) based on this.
    pub t: i32,
    /// Lasers have a direction in which they are moving (up or down);
    pub movement: Direction,
    /// How many pixels per frame the laser advances.
    pub speed: i32,
    /// What color is this laser "bullet"?
    pub color: Color,
}

/// This struct represents both the Mothership and its appearance delay.
#[derive(Clone, Serialize, Deserialize)]
pub struct Ufo {
    /// The x-coordinate of the mothership position.
    pub x: i32,
    /// The y-coordinate of the mothership position.
    pub y: i32,
    pub appearance_counter: Option<i32>,
    /// This is an animation counter; it's presence indicates the mothership has been hit and is in the process of dying.
    pub death_counter: Option<i32>,
}

/// This struct represents an enemy in Space Invaders.
#[derive(Clone, Serialize, Deserialize)]
pub struct Enemy {
    /// The enemy's current x-position.
    pub x: i32,
    /// The enemy's current y-position.
    pub y: i32,
    /// Which row does this enemy belong to?
    pub row: i32,
    /// Which column does this enemy belong to?
    pub col: i32,
    /// At what index does this enemy exist?
    pub id: u32,
    /// Is this enemy still alive?
    pub alive: bool,
    /// How many points is this enemy worth?
    pub points: i32,
    /// This is an animation counter; it's presence indicates the enemy is in the process of dying.
    pub death_counter: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnemiesMovementState {
    /// Delay between each step in movement; starts high, goes down over time.
    pub move_counter: i32,
    /// Are we moving right/left/down?
    pub move_dir: Direction,
    /// Enemies flip back and forth over time. How do they look by at current?
    pub visual_orientation: bool,
}

/// This struct represents the configuration for Space Invaders; all of these values cannot change from frame-to-frame but require a "new_game" reset to take effect.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpaceInvaders {
    /// What random numbers should be used as a seed for new games?
    pub rand: random::Gen,
    /// How many points are each enemy worth?
    pub row_scores: Vec<i32>,
    /// How many lives exist at a new game?
    pub start_lives: i32,
    /// How do enemies shoot?
    pub enemy_protocol: FiringAI,
    /// The jitter parameter represents the stochasticity of enemy firing protocols.
    pub jitter: f64,
    /// This is a list of (x,y) positions that represents where shields are created.
    pub shields: Vec<(i32, i32)>,
}

/// This struct contains the state of Space Invaders; everything that can change from frame to frame is represented.
#[derive(Clone, Serialize, Deserialize)]
pub struct StateCore {
    /// This random number generator is used for firing behavior.
    pub rand: random::Gen,
    /// This is an animation timer; lives are shown before the level begins.
    pub life_display_timer: i32,
    /// How many lives are remaining?
    pub lives: i32,
    /// How many levels have been completed?
    pub levels_completed: i32,
    /// How many points have been earned?
    pub score: i32,
    /// Ship is a rectangular actor (logically).
    pub ship: Player,
    /// Emulate the fact that Atari could only have one laser at a time (and it "recharges" faster if you hit the front row...)
    pub ship_laser: Option<Laser>,
    /// Shields are destructible, so we need to track their pixels...
    pub shields: Vec<SpriteData>,
    /// Enemies are rectangular actors (logically speaking).
    pub enemies: Vec<Enemy>,
    /// We need some variables to track the enemy movement state.
    pub enemies_movement: EnemiesMovementState,
    /// Enemy shot delay: how long between enemy shots.
    pub enemy_shot_delay: i32,
    /// The enemies can have many lasers fired at once.
    pub enemy_lasers: Vec<Laser>,

    /// Mothership
    pub ufo: Ufo,
}

/// The unified state of SpaceInvaders contains both the config (read-only) and the frame state.
pub struct State {
    /// Constant configuration available to game logic.
    pub config: SpaceInvaders,
    /// Dynamic state changes each frame.
    pub state: StateCore,
}
