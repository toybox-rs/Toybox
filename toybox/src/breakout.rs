use super::graphics::{Color, Drawable};
use super::vec2d::Vec2D;
use super::Body2D;
use super::Input;

use failure;
use serde_json;

pub mod screen {
    pub const GAME_SIZE: (i32, i32) = (480, 319);
    pub const FRAME_OFFSET: i32 = 31;
    pub const FRAME_THICKNESS: i32 = 23;
    pub const FRAME_SUPPORT_WIDTH: i32 = 24;

    pub const FRAME_RIGHT_SUPPORT: (i32, i32) = (24, 9);
    pub const FRAME_LEFT_SUPPORT: (i32, i32) = (24, 11);
    pub const FRAME_LEFT_HEIGHT: i32 = 269;
    pub const FRAME_RIGHT_HEIGHT: i32 = 267;
    pub const FRAME_LEFT_SUPPORT_COLOR: (u8, u8, u8) = (80, 156, 128);
    pub const FRAME_RIGHT_SUPPORT_COLOR: (u8, u8, u8) = (192, 88, 88);

    pub const FRAME_TO_PADDLE: i32 = 236;

    pub const FRAME_COLOR: (u8, u8, u8) = (144, 144, 144);

    pub const SCORE_CHAR_SIZE: (i32, i32) = (36, 15);

    pub const BOARD_LEFT_X: i32 = FRAME_SUPPORT_WIDTH;
    pub const BOARD_RIGHT_X: i32 = GAME_SIZE.0 - FRAME_SUPPORT_WIDTH;
    pub const BOARD_TOP_Y: i32 = FRAME_OFFSET + FRAME_THICKNESS;
    pub const BOARD_BOTTOM_Y: i32 = FRAME_OFFSET + FRAME_LEFT_HEIGHT;

    // Atari manual refers to orange, yellow, green, aqua, blue... not what images show.
    pub const RED: (u8, u8, u8) = (200, 72, 72);
    pub const DARK_ORANGE: (u8, u8, u8) = (198, 108, 58);
    pub const ORANGE: (u8, u8, u8) = (180, 122, 48);
    pub const YELLOW: (u8, u8, u8) = (162, 162, 42);
    pub const GREEN: (u8, u8, u8) = (72, 160, 72);
    pub const BLUE: (u8, u8, u8) = (66, 72, 200);

    pub const ROW_SCORES: &[u32] = &[7, 7, 7, 4, 4, 1, 1];
    pub const ROW_COLORS: &[&(u8, u8, u8)] = &[&RED, &DARK_ORANGE, &ORANGE, &YELLOW, &GREEN, &BLUE];

    // Atari colors have paddle, ball, and red all being the same.
    pub const PADDLE_COLOR: (u8, u8, u8) = (200, 72, 72);
    pub const BALL_COLOR: (u8, u8, u8) = (200, 72, 72);

    pub const ROOF_SPACING: i32 = 37;
    pub const FIELD_WIDTH: i32 = 432;
    pub const BRICK_HEIGHT: i32 = 9;
    pub const BRICK_WIDTH: i32 = 24;
    pub const BRICKS_ACROSS: i32 = (FIELD_WIDTH / BRICK_WIDTH);

    pub const PADDLE_START_Y: i32 = FRAME_TO_PADDLE + BOARD_TOP_Y;
    pub const PADDLE_START_SIZE: (i32, i32) = (48, 6);
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Brick {
    /// Brick position describes the upper-left of the brick.
    pub position: Vec2D,
    /// Brick size is the width and height of the brick.
    pub size: Vec2D,
    /// This is the number of points for a brick.
    pub points: u32,
    /// This starts as true and moves to false when hit.
    pub alive: bool,
    // What color is this brick.
    pub color: Color,
}

impl Brick {
    pub fn new(position: Vec2D, size: Vec2D, points: u32, color: Color) -> Brick {
        Brick {
            position,
            size,
            points,
            alive: true,
            color,
        }
    }

    pub fn contains(&self, point: &Vec2D) -> bool {
        point.x >= self.position.x
            && point.x <= (self.position.x + self.size.x)
            && point.y >= self.position.y
            && point.y <= (self.position.y + self.size.y)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct State {
    pub game_over: bool,
    pub points: u32,
    /// ball position describes the center of the ball.
    pub ball: Body2D,
    pub ball_radius: f64,
    /// paddle position describes the center of the paddle.
    pub paddle: Body2D,
    pub paddle_width: f64,
    pub paddle_speed: f64,
    pub bricks: Vec<Brick>,
}

pub struct Breakout;
impl super::Simulation for Breakout {
    fn game_size(&self) -> (i32, i32) {
        screen::GAME_SIZE
    }

    /// Create a new game of breakout.
    fn new_game(&self) -> Box<super::State> {
        let (w, h) = screen::GAME_SIZE;
        let mut bricks = Vec::new();
        let mut ball = Body2D::new_pos(f64::from(w) / 2.0, f64::from(h) / 2.0);
        ball.velocity = Vec2D::from_polar(4.0, 0.5);

        let offset = Vec2D::new(
            screen::BOARD_LEFT_X.into(),
            (screen::BOARD_TOP_Y + screen::ROOF_SPACING).into(),
        );
        let num_bricks_deep = screen::ROW_COLORS.len();
        let bsize = Vec2D::new(screen::BRICK_WIDTH.into(), screen::BRICK_HEIGHT.into());
        let xs = bsize.x;
        let ys = bsize.y;
        for x in 0..screen::BRICKS_ACROSS {
            let x = f64::from(x);
            for y in 0..num_bricks_deep {
                let color_tuple = screen::ROW_COLORS[y];
                let score = screen::ROW_SCORES[y];
                let bpos = Vec2D::new(x * xs, (y as f64) * ys).translate(&offset);
                bricks.push(Brick::new(bpos, bsize.clone(), score, color_tuple.into()));
            }
        }

        Box::new(State {
            game_over: false,
            ball,
            points: 0,
            ball_radius: 3.0,
            paddle: Body2D::new_pos(f64::from(w) / 2.0, screen::PADDLE_START_Y.into()),
            paddle_width: screen::PADDLE_START_SIZE.0.into(),
            paddle_speed: 4.0,
            bricks,
        })
    }

    fn new_state_from_json(&self, json_str: &str) -> Result<Box<super::State>, failure::Error> {
        let state: State = serde_json::from_str(json_str)?;
        Ok(Box::new(state))
    }
}

impl State {
    fn update_paddle_movement(&mut self, buttons: Input) {
        let left = buttons.left;
        let right = buttons.right;

        if left {
            self.paddle.velocity.x = -self.paddle_speed;
        } else if right {
            self.paddle.velocity.x = self.paddle_speed;
        } else {
            self.paddle.velocity.x = 0.0;
        }
    }
    fn update_time_slice(&mut self, time_step: f64) {
        // Update positions.
        self.ball.integrate_mut(time_step);
        self.paddle.integrate_mut(time_step);

        // Handle collisions:
        if self.ball.velocity.y > 0.0 {
            // check paddle:
            let paddle_ball_same_x = (self.ball.position.x - self.paddle.position.x).abs()
                < self.ball_radius + self.paddle_width / 2.0;
            let paddle_ball_same_y =
                (self.paddle.position.y - self.ball.position.y).abs() < self.ball_radius;
            if paddle_ball_same_x && paddle_ball_same_y {
                self.ball.velocity.y *= -1.0;
            }

            // check lose?
            if self.ball.position.y + self.ball_radius > screen::BOARD_BOTTOM_Y.into() {
                // TODO, lose
                self.game_over = true;
            }
        } else {
            // bounce ceiling?
            if self.ball.position.y - self.ball_radius < screen::BOARD_TOP_Y.into() {
                self.ball.velocity.y *= -1.0;
            }
        }

        // check living bricks:
        let ball_bounce_y = Vec2D::new(
            self.ball.position.x,
            self.ball.position.y + self.ball.velocity.y.signum() * self.ball_radius,
        );
        let ball_bounce_x = Vec2D::new(
            self.ball.position.x + self.ball.velocity.x.signum() * self.ball_radius,
            self.ball.position.y,
        );

        for brick in self.bricks.iter_mut().filter(|b| b.alive) {
            let mut hit = false;
            if brick.contains(&ball_bounce_x) {
                hit = true;
                self.ball.velocity.x *= -1.0;
            } else if brick.contains(&ball_bounce_y) {
                hit = true;
                self.ball.velocity.y *= -1.0;
            }
            if hit {
                brick.alive = false;
                self.points += brick.points;
                break;
            }
        }

        // bounce right wall?
        if self.ball.velocity.x > 0.0 {
            if self.ball.position.x + self.ball_radius > screen::BOARD_RIGHT_X.into() {
                self.ball.velocity.x *= -1.0;
            }
        } else {
            // bounce left wall?
            if self.ball.position.x - self.ball_radius < screen::BOARD_LEFT_X.into() {
                self.ball.velocity.x *= -1.0;
            }
        }
    }
}

impl super::State for State {
    fn game_over(&self) -> bool {
        self.game_over
    }

    /// Mutably update the game state.
    fn update_mut(&mut self, buttons: Input) {
        self.update_paddle_movement(buttons);

        let distance_limit = self.ball_radius as i32;
        let total_time = 1.0;
        let distance_limit = distance_limit as f64; // m
        let speed = self.ball.velocity.magnitude(); // m/s

        // if your speed is 30, and your radius is 5, we want to do about 6 steps.
        let time_step = distance_limit / speed; // (m) / (m/s) = m * s / m = s

        // Update positions, checking for collisions in as many increments as is needed.
        let mut time_simulated = 0.0;
        while time_simulated < 1.0 {
            let time_left = total_time - time_simulated;
            if time_left < time_step {
                self.update_time_slice(time_left);
                return;
            } else {
                self.update_time_slice(time_step);
                time_simulated += time_step;
            }
        }
    }

    fn draw(&self) -> Vec<Drawable> {
        let mut output = Vec::new();
        output.push(Drawable::rect(
            Color::black(),
            0,
            0,
            screen::GAME_SIZE.0,
            screen::GAME_SIZE.1,
        ));

        // Draw frame top:
        output.push(Drawable::rect(
            (&screen::FRAME_COLOR).into(),
            0,
            screen::FRAME_OFFSET,
            screen::GAME_SIZE.0,
            screen::FRAME_THICKNESS,
        ));

        // Draw frame left:
        output.push(Drawable::rect(
            (&screen::FRAME_COLOR).into(),
            0,
            screen::FRAME_OFFSET,
            screen::FRAME_SUPPORT_WIDTH,
            screen::FRAME_LEFT_HEIGHT,
        ));

        // Draw frame left "colored spot"
        output.push(Drawable::rect(
            (&screen::FRAME_LEFT_SUPPORT_COLOR).into(),
            0,
            screen::FRAME_OFFSET + screen::FRAME_LEFT_HEIGHT - screen::FRAME_LEFT_SUPPORT.1,
            screen::FRAME_LEFT_SUPPORT.0,
            screen::FRAME_LEFT_SUPPORT.1,
        ));

        // Draw frame right:
        output.push(Drawable::rect(
            (&screen::FRAME_COLOR).into(),
            screen::BOARD_RIGHT_X,
            screen::FRAME_OFFSET,
            screen::FRAME_SUPPORT_WIDTH,
            screen::FRAME_RIGHT_HEIGHT,
        ));

        // Draw frame right "colored spot"
        output.push(Drawable::rect(
            (&screen::FRAME_RIGHT_SUPPORT_COLOR).into(),
            screen::BOARD_RIGHT_X,
            screen::FRAME_OFFSET + screen::FRAME_RIGHT_HEIGHT - screen::FRAME_RIGHT_SUPPORT.1,
            screen::FRAME_RIGHT_SUPPORT.0,
            screen::FRAME_RIGHT_SUPPORT.1,
        ));

        if self.game_over {
            return output;
        }

        for brick in self.bricks.iter().filter(|b| b.alive) {
            let (x, y) = brick.position.pixels();
            let (w, h) = brick.size.pixels();

            output.push(Drawable::rect(brick.color, x, y, w, h));
        }

        let (paddle_x, paddle_y) = self.paddle.position.pixels();
        let paddle_w = self.paddle_width as i32;

        output.push(Drawable::rect(
            (&screen::PADDLE_COLOR).into(),
            paddle_x - paddle_w / 2,
            paddle_y,
            paddle_w,
            screen::PADDLE_START_SIZE.1,
        ));

        let (ball_x, ball_y) = self.ball.position.pixels();
        let ball_r = self.ball_radius as i32;
        output.push(Drawable::rect(
            (&screen::BALL_COLOR).into(),
            ball_x - ball_r,
            ball_y - ball_r,
            ball_r * 2,
            ball_r * 2,
        ));

        output
    }
    
    fn to_json(&self) -> String {
        serde_json::to_string(self).expect("Should be no JSON Serialization Errors.")
    }
}
