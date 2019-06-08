use super::body2d::Body2D;
use super::font::{draw_lives, draw_score, DIGIT_WIDTH};
use super::vec2d::Vec2D;
use ordered_float::NotNan;
use toybox_core;
use toybox_core::graphics::{Color, Drawable};
use toybox_core::random;
use toybox_core::{AleAction, Input, QueryError};

use serde_json;

use types::*;

use rand::seq::SliceRandom;

/// This module contains constants derived from observation and measurement of the Atari 2600 game.
mod screen {
    /// This is the size of the screen.
    pub const GAME_SIZE: (i32, i32) = (240, 160);
    /// This is the y-offset of the gray frame that surrounds the board.
    pub const FRAME_OFFSET: i32 = 13;
    /// This is the thickness of the gray frame that surrounds the board.
    pub const FRAME_THICKNESS: i32 = 12;
    pub const FRAME_SUPPORT_WIDTH: i32 = 12;

    pub const FRAME_RIGHT_SUPPORT: (i32, i32) = (12, 4);
    pub const FRAME_LEFT_SUPPORT: (i32, i32) = (12, 4);
    pub const FRAME_LEFT_HEIGHT: i32 = 135;
    pub const FRAME_RIGHT_HEIGHT: i32 = 135;
    pub const FRAME_LEFT_SUPPORT_COLOR: (u8, u8, u8) = (80, 156, 128);
    pub const FRAME_RIGHT_SUPPORT_COLOR: (u8, u8, u8) = (192, 88, 88);

    pub const FRAME_TO_PADDLE: i32 = 118;

    pub const FRAME_COLOR: (u8, u8, u8) = (144, 144, 144);

    pub const BOARD_LEFT_X: i32 = FRAME_SUPPORT_WIDTH;
    pub const BOARD_RIGHT_X: i32 = GAME_SIZE.0 - FRAME_SUPPORT_WIDTH;
    pub const BOARD_TOP_Y: i32 = FRAME_OFFSET + FRAME_THICKNESS;
    pub const BOARD_BOTTOM_Y: i32 = FRAME_OFFSET + FRAME_LEFT_HEIGHT;

    // Atari manual refers to orange, yellow, green, aqua, blue... not what images show.
    /// This is the RGB color of the red bricks.
    pub const RED: (u8, u8, u8) = (200, 72, 72);
    /// This is the RGB color of the dark-orange bricks.
    pub const DARK_ORANGE: (u8, u8, u8) = (198, 108, 58);
    /// This is the RGB color of the orange bricks.
    pub const ORANGE: (u8, u8, u8) = (180, 122, 48);
    /// This is the RGB color of the yellow bricks.
    pub const YELLOW: (u8, u8, u8) = (162, 162, 42);
    /// This is the RGB color of the green bricks.
    pub const GREEN: (u8, u8, u8) = (72, 160, 72);
    /// This is the RGB color of the blue bricks.
    pub const BLUE: (u8, u8, u8) = (66, 72, 200);

    /// This is the point value of the rows of bricks, from top to bottom.
    pub const ROW_SCORES: &[i32] = &[7, 7, 4, 4, 1, 1];
    /// This is the color of the rows of bricks, from top to bottom.
    pub const ROW_COLORS: &[&(u8, u8, u8)] = &[&RED, &DARK_ORANGE, &ORANGE, &YELLOW, &GREEN, &BLUE];

    // Atari colors have paddle, ball, and red all being the same.
    /// The color of the paddle in the atari-py version.
    pub const PADDLE_COLOR: (u8, u8, u8) = (200, 72, 72);
    /// The color of the ball in the atari-py version.
    pub const BALL_COLOR: (u8, u8, u8) = (200, 72, 72);

    pub const ROOF_SPACING: i32 = 18;
    pub const FIELD_WIDTH: i32 = 216;
    pub const BRICK_HEIGHT: i32 = 4;
    pub const BRICK_WIDTH: i32 = 12;
    pub const BRICKS_ACROSS: i32 = (FIELD_WIDTH / BRICK_WIDTH);

    pub const PADDLE_START_Y: i32 = FRAME_TO_PADDLE + BOARD_TOP_Y;
    pub const PADDLE_START_SIZE: (i32, i32) = (24, 3);
    pub const PADDLE_SMALL_SIZE: (i32, i32) = (16, 3);

    pub const BALL_ANGLE_MIN: f64 = 30.0;
    pub const BALL_ANGLE_RANGE: f64 = 120.0;
}

impl Breakout {
    #[cfg(test)]
    fn unique_colors(&self) -> Vec<&Color> {
        let mut output: Vec<&Color> = Vec::new();
        output.extend(self.row_colors.iter());
        output.push(&self.bg_color);
        output.push(&self.frame_color);
        // Note, ball and paddle are the same and that's OK for breakout.
        output
    }
    fn start_paddle(&self) -> Body2D {
        let (w, _h) = screen::GAME_SIZE;
        Body2D::new_pos(f64::from(w) / 2.0, screen::PADDLE_START_Y.into())
    }
}

impl Default for Breakout {
    fn default() -> Self {
        let (w, h) = screen::GAME_SIZE;
        let w = w as f64;
        let y = h as f64 / 2.0;

        Breakout {
            rand: random::Gen::new_from_seed(13),
            bg_color: Color::black(),
            frame_color: (&screen::FRAME_COLOR).into(),
            paddle_color: (&screen::PADDLE_COLOR).into(),
            ball_color: (&screen::BALL_COLOR).into(),
            row_colors: screen::ROW_COLORS
                .iter()
                .cloned()
                .map(|c| c.into())
                .collect(),
            row_scores: screen::ROW_SCORES.to_vec(),
            start_lives: 5,
            ball_speed_row_depth: 3, // orange is 0..1..2..3
            ball_speed_slow: 2.0,
            ball_speed_fast: 4.0,
            ball_start_positions: vec![
                StartBall::new(0.1 * w, y, 30.0),
                StartBall::new(0.5 * w, y, 30.0),
                StartBall::new(0.5 * w, y, 150.0),
                StartBall::new(0.9 * w, y, 150.0),
            ],
            paddle_discrete_segments: Some(5),
        }
    }
}

impl Brick {
    pub fn new(
        row: i32,
        col: i32,
        position: Vec2D,
        size: Vec2D,
        points: i32,
        color: Color,
        depth: u32,
    ) -> Brick {
        Brick {
            row,
            col,
            position,
            size,
            points,
            color,
            depth,
            alive: true,
            destructible: true,
        }
    }

    /// Now that we have non-breakable bricks, we can use this everywhere to tell if a brick is completed or not.
    pub fn completed(&self) -> bool {
        if self.destructible {
            !self.alive
        } else {
            true
        }
    }

    pub fn contains(&self, point: &Vec2D) -> bool {
        point.x >= self.position.x
            && point.x <= (self.position.x + self.size.x)
            && point.y >= self.position.y
            && point.y <= (self.position.y + self.size.y)
    }
}

impl toybox_core::Simulation for Breakout {
    fn reset_seed(&mut self, seed: u32) {
        self.rand.reset_seed(seed);
    }
    fn game_size(&self) -> (i32, i32) {
        screen::GAME_SIZE
    }

    /// Sync with [ALE Impl](https://github.com/mgbellemare/Arcade-Learning-Environment/blob/master/src/games/supported/Breakout.cpp#L80)
    /// Note, leaving a call to sort in this impl to remind users that these vecs are ordered!
    fn legal_action_set(&self) -> Vec<AleAction> {
        let mut actions = vec![
            AleAction::NOOP,
            AleAction::FIRE,
            AleAction::LEFT,
            AleAction::RIGHT,
        ];
        actions.sort();
        actions
    }

    /// Create a new game of breakout.
    fn new_game(&mut self) -> Box<toybox_core::State> {
        let mut bricks = Vec::new();

        let offset = Vec2D::new(
            screen::BOARD_LEFT_X.into(),
            (screen::BOARD_TOP_Y + screen::ROOF_SPACING).into(),
        );
        let num_bricks_deep = self.row_colors.len();
        let bsize = Vec2D::new(screen::BRICK_WIDTH.into(), screen::BRICK_HEIGHT.into());
        let xs = bsize.x;
        let ys = bsize.y;
        for xi in 0..screen::BRICKS_ACROSS {
            let x = f64::from(xi);
            for yi in 0..num_bricks_deep {
                let color_tuple = self.row_colors[yi];
                let score = self.row_scores[yi];
                let bpos = Vec2D::new(x * xs, (yi as f64) * ys).translate(&offset);
                // Reverse depth:
                let depth = num_bricks_deep - yi - 1;
                bricks.push(Brick::new(
                    yi as i32,
                    xi,
                    bpos,
                    bsize.clone(),
                    score,
                    color_tuple,
                    depth as u32,
                ));
            }
        }

        let mut state = State {
            config: self.clone(),
            state: StateCore {
                lives: self.start_lives,
                // empty to start
                balls: Vec::new(),
                is_dead: true,
                // paddle starts in middle
                paddle: self.start_paddle(),
                points: 0,
                ball_radius: 2.0,
                paddle_width: screen::PADDLE_START_SIZE.0.into(),
                paddle_speed: 4.0,
                rand: random::Gen::new_child(&mut self.rand),
                bricks,
                reset: true,
            },
        };

        state.start_ball();
        Box::new(state)
    }

    fn new_state_from_json(
        &self,
        json_str: &str,
    ) -> Result<Box<toybox_core::State>, serde_json::Error> {
        let state: StateCore = serde_json::from_str(json_str)?;
        Ok(Box::new(State {
            config: self.clone(),
            state,
        }))
    }

    fn from_json(&self, json_str: &str) -> Result<Box<toybox_core::Simulation>, serde_json::Error> {
        let config: Breakout = serde_json::from_str(json_str)?;
        Ok(Box::new(config))
    }

    fn to_json(&self) -> String {
        serde_json::to_string(self).expect("Breakout should be JSON-serializable!")
    }
}

impl State {
    fn start_ball(&mut self) {
        let option: &StartBall = self
            .config
            .ball_start_positions
            .choose(&mut self.state.rand)
            .unwrap();

        let mut ball = Body2D::new_pos(option.x, option.y);
        ball.velocity = Vec2D::from_polar(
            self.config.ball_speed_slow,
            option.angle_degrees.to_radians(),
        );
        self.state.balls.push(ball);
    }
    fn update_paddle_movement(&mut self, buttons: Input) {
        let left = buttons.left;
        let right = buttons.right;

        if left {
            self.state.paddle.velocity.x = -self.state.paddle_speed;
        } else if right {
            self.state.paddle.velocity.x = self.state.paddle_speed;
        } else {
            self.state.paddle.velocity.x = 0.0;
        }
    }
    fn keep_paddle_on_screen(&mut self) {
        let left = screen::BOARD_LEFT_X as f64 - self.state.paddle_width / 2.0;
        let right = screen::BOARD_RIGHT_X as f64 + self.state.paddle_width / 2.0;
        if self.state.paddle.position.x < left {
            self.state.paddle.position.x = left;
            self.state.paddle.velocity.x = 0.0;
        } else if self.state.paddle.position.x > right {
            self.state.paddle.position.x = right;
            self.state.paddle.velocity.x = 0.0;
        }
    }
    fn check_bounce_paddle(&mut self) {
        let radius = self.state.ball_radius;

        for ball in self.state.balls.iter_mut() {
            // Only check balls going downwards.
            if ball.velocity.y <= 0.0 {
                continue;
            }

            // check paddle:
            let paddle_ball_same_x = (ball.position.x - self.state.paddle.position.x).abs()
                < radius + self.state.paddle_width / 2.0;
            let paddle_ball_same_y =
                (self.state.paddle.position.y - ball.position.y).abs() < radius;

            if paddle_ball_same_x && paddle_ball_same_y {
                // get x location of ball hit relative to paddle
                let ball_hit_x = ball.position.x
                    - (self.state.paddle.position.x - (self.state.paddle_width / 2.0));
                // get normalized location of ball hit along paddle
                let mut paddle_normalized_relative_intersect_x =
                    1.0 - ball_hit_x / self.state.paddle_width;

                // If we have discrete segments, discretize that.
                if let Some(segments) = self.config.paddle_discrete_segments {
                    // Multiply to get an integer segment id.
                    let segment_id =
                        (paddle_normalized_relative_intersect_x * segments as f64).floor() as i32;
                    // Center within segments.
                    let shift = 1.0 / (2.0 * segments as f64);
                    // Divide to go back to a number from 0..1.0
                    let relative = (segment_id as f64) / (segments as f64) + shift;
                    // Overwrite continuous value.
                    paddle_normalized_relative_intersect_x = relative;
                }

                // convert this normalized parameter to the degree of the bounce angle
                let bounce_angle = paddle_normalized_relative_intersect_x
                    * screen::BALL_ANGLE_RANGE
                    + screen::BALL_ANGLE_MIN;

                ball.velocity =
                    Vec2D::from_polar(ball.velocity.magnitude(), bounce_angle.to_radians());
                // calculations use non-graphics polar orientation
                // to quickly fix, we reflect over the x-axis
                ball.velocity.y *= -1.0;
            }
        }
    }

    fn check_ball_death(&mut self) -> bool {
        let radius = self.state.ball_radius;

        let mut died = Vec::new();
        for (i, ball) in self.state.balls.iter_mut().enumerate() {
            // Only those balls downward:
            if ball.velocity.y < 0.0 {
                continue;
            }
            if ball.position.y + radius > screen::BOARD_BOTTOM_Y.into() {
                died.push(i);
            }
        }
        for index in died.iter().rev() {
            self.state.balls.remove(*index);
        }

        // Death when no more balls!
        self.state.balls.is_empty()
    }

    fn update_time_slice(&mut self, time_step: f64) {
        // Update positions.

        for ball in self.state.balls.iter_mut() {
            ball.integrate_mut(time_step);
        }
        self.state.paddle.integrate_mut(time_step);

        self.keep_paddle_on_screen();
        self.check_bounce_paddle();

        // check lose?
        if self.check_ball_death() && !self.state.is_dead {
            if !self.state.is_dead {
                self.state.lives -= 1;
                self.state.paddle_width = screen::PADDLE_START_SIZE.0.into();
            }
            self.state.is_dead = true;
            return;
        }

        for ball in self.state.balls.iter_mut() {
            let radius = self.state.ball_radius;

            // Handle collisions:
            if ball.velocity.y < 0.0 {
                // bounce ceiling?
                if ball.position.y - radius < screen::BOARD_TOP_Y.into() {
                    ball.velocity.y *= -1.0;
                    self.state.paddle_width = screen::PADDLE_SMALL_SIZE.0.into();
                }
            }

            // check living bricks:
            let ball_bounce_y = Vec2D::new(
                ball.position.x,
                ball.position.y + ball.velocity.y.signum() * radius,
            );
            let ball_bounce_x = Vec2D::new(
                ball.position.x + ball.velocity.x.signum() * radius,
                ball.position.y,
            );

            for brick in self.state.bricks.iter_mut().filter(|b| b.alive) {
                let mut hit = false;
                if brick.contains(&ball_bounce_x) {
                    hit = true;
                    ball.velocity.x *= -1.0;
                } else if brick.contains(&ball_bounce_y) {
                    hit = true;
                    ball.velocity.y *= -1.0;
                }
                if hit {
                    if brick.destructible {
                        brick.alive = false;
                        self.state.points += brick.points;
                    }
                    if brick.depth >= self.config.ball_speed_row_depth {
                        // Potentially speed up the ball. This will be a no-op if it's already fast.
                        let theta = ball.velocity.angle();
                        ball.velocity = Vec2D::from_polar(self.config.ball_speed_fast, theta);
                    }
                    break;
                }
            }

            // bounce right wall?
            if ball.velocity.x > 0.0 {
                if ball.position.x + radius > screen::BOARD_RIGHT_X.into() {
                    ball.velocity.x *= -1.0;
                }
            } else {
                // bounce left wall?
                if ball.position.x - radius < screen::BOARD_LEFT_X.into() {
                    ball.velocity.x *= -1.0;
                }
            }
        }
    }
}

impl toybox_core::State for State {
    fn lives(&self) -> i32 {
        self.state.lives
    }
    fn score(&self) -> i32 {
        self.state.points
    }

    /// Mutably update the game state.
    fn update_mut(&mut self, buttons: Input) {
        self.update_paddle_movement(buttons);

        if self.state.is_dead {
            if buttons.button1 {
                self.start_ball();
                self.state.is_dead = false;
            }
        }

        if buttons.button2 {
            self.start_ball();
        }

        let distance_limit = self.state.ball_radius as i32;
        let total_time = 1.0;
        let distance_limit = distance_limit as f64; // m
        let speed: f64 = self
            .state
            .balls
            .iter()
            .map(|ball| NotNan::new(ball.velocity.magnitude()).expect("NaN velocity magnitude!"))
            .max()
            .map(|nn| nn.into_inner())
            .unwrap_or(distance_limit); // m/s

        // if your speed is 30, and your radius is 5, we want to do about 6 steps.
        let time_step = distance_limit / speed; // (m) / (m/s) = m * s / m = s

        // Update positions, checking for collisions in as many increments as is needed.
        let mut time_simulated = 0.0;
        while time_simulated < 1.0 {
            let time_left = total_time - time_simulated;
            if time_left < time_step {
                self.update_time_slice(time_left);
                break;
            } else {
                self.update_time_slice(time_step);
                if self.state.is_dead {
                    // Don't simulate if dead.
                    break;
                }
                time_simulated += time_step;
            }
        }

        let reset_level = self.state.bricks.iter().all(|b| b.completed());
        if reset_level && self.state.reset {
            for b in self.state.bricks.iter_mut() {
                b.alive = true;
            }
            // Delete old ball(s).
            self.state.balls.clear();
            // New ball.
            self.start_ball();
            self.state.reset = false;
        }
    }

    fn draw(&self) -> Vec<Drawable> {
        let mut output = Vec::new();
        output.push(Drawable::Clear(self.config.bg_color));

        // Draw frame top:
        output.push(Drawable::rect(
            self.config.frame_color,
            0,
            screen::FRAME_OFFSET,
            screen::GAME_SIZE.0,
            screen::FRAME_THICKNESS,
        ));

        // Draw frame left:
        output.push(Drawable::rect(
            self.config.frame_color,
            0,
            screen::FRAME_OFFSET,
            screen::FRAME_SUPPORT_WIDTH,
            screen::FRAME_LEFT_HEIGHT,
        ));

        // Draw frame right:
        output.push(Drawable::rect(
            (&screen::FRAME_COLOR).into(),
            screen::BOARD_RIGHT_X,
            screen::FRAME_OFFSET,
            screen::FRAME_SUPPORT_WIDTH,
            screen::FRAME_RIGHT_HEIGHT,
        ));

        // Draw frame left "colored spot"
        output.push(Drawable::rect(
            (&screen::FRAME_LEFT_SUPPORT_COLOR).into(),
            0,
            screen::FRAME_OFFSET + screen::FRAME_LEFT_HEIGHT - screen::FRAME_LEFT_SUPPORT.1,
            screen::FRAME_LEFT_SUPPORT.0,
            screen::FRAME_LEFT_SUPPORT.1,
        ));

        // Draw frame right "colored spot"
        output.push(Drawable::rect(
            (&screen::FRAME_RIGHT_SUPPORT_COLOR).into(),
            screen::BOARD_RIGHT_X,
            screen::FRAME_OFFSET + screen::FRAME_RIGHT_HEIGHT - screen::FRAME_RIGHT_SUPPORT.1,
            screen::FRAME_RIGHT_SUPPORT.0,
            screen::FRAME_RIGHT_SUPPORT.1,
        ));

        if self.state.lives < 0 {
            return output;
        }

        for brick in self.state.bricks.iter().filter(|b| b.alive) {
            let (x, y) = brick.position.pixels();
            let (w, h) = brick.size.pixels();

            output.push(Drawable::rect(brick.color, x, y, w, h));
        }

        let (paddle_x, paddle_y) = self.state.paddle.position.pixels();
        let paddle_w = self.state.paddle_width as i32;

        output.push(Drawable::rect(
            self.config.paddle_color,
            paddle_x - paddle_w / 2,
            paddle_y,
            paddle_w,
            screen::PADDLE_START_SIZE.1,
        ));

        let ball_r = self.state.ball_radius as i32;
        for ball in self.state.balls.iter() {
            let (ball_x, ball_y) = ball.position.pixels();
            output.push(Drawable::rect(
                self.config.ball_color,
                ball_x - ball_r,
                ball_y - ball_r,
                ball_r * 2,
                ball_r * 2,
            ));
        }

        let score_offset = 88;
        let score_x = screen::BOARD_LEFT_X + score_offset;
        let lives_x = score_x + (DIGIT_WIDTH * 2);
        let thing_x = lives_x + (DIGIT_WIDTH * 2);
        // Draw points:
        output.extend(draw_score(self.state.points, score_x, 1));
        // Draw lives:
        output.extend(draw_lives(self.state.lives, lives_x, 1));
        // Draw whatever this thing is
        output.extend(draw_lives(1, thing_x, 1));

        output
    }

    fn to_json(&self) -> String {
        serde_json::to_string(&self.state).expect("Should be no JSON Serialization Errors.")
    }

    fn query_json(&self, query: &str, args: &serde_json::Value) -> Result<String, QueryError> {
        let config = &self.config;
        let state = &self.state;
        Ok(match query {
            "bricks_remaining" => {
                serde_json::to_string(&state.bricks.iter().filter(|b| !b.completed()).count())?
            }
            "brick_live_by_index" => {
                if let Some(index) = args.as_u64() {
                    serde_json::to_string(&!state.bricks[index as usize].completed())?
                } else {
                    Err(QueryError::BadInputArg)?
                }
            }
            "count_channels" => serde_json::to_string(&state.find_channels().len())?,
            "channels" => serde_json::to_string(&state.find_channels())?,
            "num_columns" => serde_json::to_string(&screen::BRICKS_ACROSS)?,
            "num_rows" => serde_json::to_string(&screen::ROW_SCORES.len())?,
            "config.ball_start_positions" => serde_json::to_string(&config.ball_start_positions)?,
            _ => Err(QueryError::NoSuchQuery)?,
        })
    }
}

/// Define some queries on StateCore.
impl StateCore {
    /// Returns a set of numbers corresponding to the stacks that are channels.
    fn find_channels(&self) -> Vec<i32> {
        let across = screen::BRICKS_ACROSS as i32;
        let down = screen::ROW_SCORES.len() as i32;
        let mut retval = Vec::new();

        for offset in 0..across {
            let all_dead = (0..down)
                .map(|row| {
                    let i = row + offset * down;
                    !self.bricks[i as usize].alive
                })
                .all(|c| c);
            if all_dead {
                retval.push(offset);
                assert!(retval.len() <= (across as usize));
            }
        }
        assert!(retval.len() <= (across as usize));
        retval
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;
    use toybox_core::Simulation;

    #[test]
    fn test_colors_unique_in_gray() {
        let config = Breakout::default();
        let num_colors = config.unique_colors().len();
        let uniq_grays: HashSet<u8> = config
            .unique_colors()
            .into_iter()
            .map(|c| c.grayscale_byte())
            .collect();
        // Don't allow a grayscale agent to be confused where a human wouldn't be.
        assert_eq!(uniq_grays.len(), num_colors);
    }

    #[test]
    fn test_q_breakout_bricks_remaining() {
        let mut breakout = super::Breakout::default();
        let state = breakout.new_game();
        let bricks_remaining = state
            .query_json("bricks_remaining", &serde_json::Value::Null)
            .unwrap()
            .parse::<u32>()
            .unwrap();
        let num_columns = state
            .query_json("num_columns", &serde_json::Value::Null)
            .unwrap()
            .parse::<u32>()
            .unwrap();
        let num_rows = state
            .query_json("num_rows", &serde_json::Value::Null)
            .unwrap()
            .parse::<u32>()
            .unwrap();

        assert_eq!(bricks_remaining, num_columns * num_rows);
    }

    #[test]
    fn test_q_breakout_channels() {
        let mut breakout = super::Breakout::default();
        let state = breakout.new_game();

        let empty = state
            .query_json("channels", &serde_json::Value::Null)
            .unwrap();
        assert_eq!(empty, "[]");
    }
}
