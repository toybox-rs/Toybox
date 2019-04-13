use super::body2d::Body2D;
use super::vec2d::Vec2D;
use toybox_core::graphics::Color;
use toybox_core::random;

/// Breakout is configured to sample randomly from ball starting positions. This struct contains all the information needed to add a new option.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StartBall {
    /// Where does the ball start? Horizontal positioning.
    pub x: f64,
    /// Where does the ball start? Vertical positioning.
    pub y: f64,
    /// At what angle is the ball moving, intitially.
    pub angle_degrees: f64,
}

impl StartBall {
    /// Constructor for a new starting ball configuration.
    pub fn new(x: f64, y: f64, angle_degrees: f64) -> StartBall {
        StartBall {
            x,
            y,
            angle_degrees,
        }
    }
}

/// This struct represents all the static data needed to create a new game of Breakout.
/// The data in this struct represents the Toybox config for this game.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Breakout {
    /// Random number generator used to seed new games. The randomness in breakout is derived from the starting ball configurations.
    pub rand: random::Gen,
    /// What is the background color of the board? Black by default.
    pub bg_color: Color,
    /// The gray area surronding the board we refer to as the "frame".
    pub frame_color: Color,
    /// What color is the Breakout paddle?
    pub paddle_color: Color,
    /// What color are the balls in Breakout?
    pub ball_color: Color,
    /// Each row has its own color in Breakout. These can be configured here.
    pub row_colors: Vec<Color>,
    /// Each row has its own score in Breakout. These can be configured here.
    pub row_scores: Vec<i32>,
    /// How many lives or balls do you start a new game with?
    pub start_lives: i32,
    /// Upon destroying a brick of a certain depth, the ball speed increases. This represents that depth, measured from the top of the board.
    pub ball_speed_row_depth: u32,
    /// How fast does the ball move when it's in "slow" mode, or at the start of a game.
    pub ball_speed_slow: f64,
    /// How fast should the ball move after breaking through to the ``ball_speed_row_depth`` level?
    pub ball_speed_fast: f64,
    /// What starting configurations are available to the game? One is chosen at random when you die or start a new game.
    pub ball_start_positions: Vec<StartBall>,
    /// When this is None, the paddle uses continuous logic for bouncing (imagining the paddle is kind of a circle). In the real game, some discrete math was used; i.e. the paddle behaves like a n-polygon. This could affect learning speed.
    pub paddle_discrete_segments: Option<i32>,
}

/// This data structure represents a Brick in the breakout game. Bricks are present in state even if they are destroyed, thus the presence of the "alive" boolean.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Brick {
    // Logical y-coordinates of this brick; used for analysis.
    pub row: i32,
    // Logical x-coordinates of this brick; used for analysis.
    pub col: i32,
    /// Brick position describes the upper-left of the brick.
    pub position: Vec2D,
    /// Brick size is the width and height of the brick.
    pub size: Vec2D,
    /// This is the number of points for a brick.
    pub points: i32,
    /// This starts as true and moves to false when hit.
    pub alive: bool,
    /// What color is this brick.
    pub color: Color,
    /// How deep is this brick? Will trigger speedup?
    pub depth: u32,
    /// Destructible: if false, never let this brick die.
    pub destructible: bool,
}

/// This struct contains the per-frame snapshot of mutable state in a Breakout game.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StateCore {
    /// This random number generator is used to select the starting position and angle of the ball.
    pub rand: random::Gen,
    /// Lives decrease every time the paddle misses the ball.
    pub lives: i32,
    /// The game does not proceed until the user presses the FIRE button to dispatch a new ball.
    pub is_dead: bool,
    /// How many points has the player earned?
    pub points: i32,
    /// Ball position describes the center of the ball.
    pub balls: Vec<Body2D>,
    /// How large is the ball? The ball is rendered as a squre and physics is calculated based on this.
    pub ball_radius: f64,
    /// Paddle position describes the center of the paddle.
    pub paddle: Body2D,
    /// How wide is the paddle?
    pub paddle_width: f64,
    /// How fast does the paddle move? When the LEFT button is pressed, this will affect the paddle position.
    pub paddle_speed: f64,
    /// Bricks are available in a flat list.
    pub bricks: Vec<Brick>,
    /// When set to true (from beating the level or dying), the bricks are reset to alive and a new ball is generated.
    pub reset: bool,
}

/// The breakout game's true state has both the configuration that launched the game and information about the current frame.
pub struct State {
    /// This contains information about the game that does not change during gameplay, but is referenced, read-only.
    pub config: Breakout,
    /// This contains information about the current snapshot of game state.
    pub state: StateCore,
}
