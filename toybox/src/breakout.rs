use super::graphics::{Color, Drawable};
use super::vec2d::Vec2D;
use super::Body2D;
use super::Input;
use super::digit_sprites::draw_score;
use super::random;

use failure;
use serde_json;

pub mod screen {
    pub const GAME_SIZE: (i32, i32) = (240, 160);
    pub const FRAME_OFFSET: i32 = 15;
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

    pub const SCORE_CHAR_SIZE: (i32, i32) = (18, 7);

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

    pub const ROW_SCORES: &[i32] = &[7, 7, 7, 4, 4, 1, 1];
    pub const ROW_COLORS: &[&(u8, u8, u8)] = &[&RED, &DARK_ORANGE, &ORANGE, &YELLOW, &GREEN, &BLUE];

    // Atari colors have paddle, ball, and red all being the same.
    pub const PADDLE_COLOR: (u8, u8, u8) = (200, 72, 72);
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
    pub const BALL_SPEED_START: f64 = 2.0;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    bg_color: Color,
    frame_color: Color,
    paddle_color: Color,
    ball_color: Color,
    row_colors: Vec<Color>,
    row_scores: Vec<i32>,
    start_lives: i32,
    ball_speed_row_depth: u32, 
    ball_speed_slow: f64,
    ball_speed_fast: f64
}
impl Config {
    fn unique_colors(&self) -> Vec<&Color> {
        let mut output: Vec<&Color> = Vec::new();
        output.extend(self.row_colors.iter());
        output.push(&self.bg_color);
        output.push(&self.frame_color);
        // Note, ball and paddle are the same and that's OK for breakout.
        output
    }
    fn start_paddle(&self) -> Body2D {
        let (w,h) = screen::GAME_SIZE;
        Body2D::new_pos(f64::from(w) / 2.0, screen::PADDLE_START_Y.into())
    }
}
impl Default for Config {
    fn default() -> Self {
        Config {
            bg_color: Color::black(),
            frame_color: (&screen::FRAME_COLOR).into(),
            paddle_color: (&screen::PADDLE_COLOR).into(),
            ball_color: (&screen::BALL_COLOR).into(),
            row_colors: screen::ROW_COLORS.iter().cloned().map(|c| c.into()).collect(),
            row_scores: screen::ROW_SCORES.iter().cloned().collect(),
            start_lives: 5,
            ball_speed_row_depth: 3, // orange is 0..1..2..3
            ball_speed_slow: 2.0,
            ball_speed_fast: 4.0,
        }
    }
}


#[derive(Debug, Clone, Serialize, Deserialize)]
#[repr(C)]
pub struct Brick {
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
}

impl Brick {
    pub fn new(position: Vec2D, size: Vec2D, points: i32, color: Color, depth: u32) -> Brick {
        Brick {
            position,
            size,
            points,
            alive: true,
            color,
            depth,
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
#[repr(C)]
pub struct State {
    pub config: Config,
    pub rand: random::Gen,
    pub lives: i32,
    pub is_dead: bool,
    pub points: i32,
    /// ball position describes the center of the ball.
    pub ball: Body2D,
    pub ball_radius: f64,
    /// paddle position describes the center of the paddle.
    pub paddle: Body2D,
    pub paddle_width: f64,
    pub paddle_speed: f64,
    pub bricks: Vec<Brick>,
}

pub struct Breakout {
    pub config: Config,
    pub rand: random::Gen,
}
impl Default for Breakout {
    fn default() -> Self {
        Breakout { config: Config::default(), rand: random::Gen::new_from_seed(13) }
    }
}

impl super::Simulation for Breakout {
    fn reset_seed(&mut self, seed: u32) {
        self.rand.reset_seed(seed);
    }
    fn game_size(&self) -> (i32, i32) {
        screen::GAME_SIZE
    }

    /// Create a new game of breakout.
    fn new_game(&mut self) -> Box<super::State> {
        let (w, h) = screen::GAME_SIZE;
        let mut bricks = Vec::new();

        let offset = Vec2D::new(
            screen::BOARD_LEFT_X.into(),
            (screen::BOARD_TOP_Y + screen::ROOF_SPACING).into(),
        );
        let num_bricks_deep = self.config.row_colors.len();
        let bsize = Vec2D::new(screen::BRICK_WIDTH.into(), screen::BRICK_HEIGHT.into());
        let xs = bsize.x;
        let ys = bsize.y;
        for x in 0..screen::BRICKS_ACROSS {
            let x = f64::from(x);
            for y in 0..num_bricks_deep {
                let color_tuple = self.config.row_colors[y];
                let score = self.config.row_scores[y];
                let bpos = Vec2D::new(x * xs, (y as f64) * ys).translate(&offset);
                // Reverse depth:
                let depth = num_bricks_deep - y - 1;
                bricks.push(Brick::new(bpos, bsize.clone(), score, color_tuple.into(), depth as u32));
            }
        }

        let mut state = State {
            config: self.config.clone(),
            lives: self.config.start_lives,
            // offscreen, and dead
            ball: Body2D::new_pos(-100.0, -100.0),
            is_dead: true,
            // paddle starts in middle
            paddle: self.config.start_paddle(),
            points: 0,
            ball_radius: 2.0,
            paddle_width: screen::PADDLE_START_SIZE.0.into(),
            paddle_speed: 4.0,
            rand: random::Gen::new_child(&mut self.rand),
            bricks,
        };
        state.start_ball();
        Box::new(state)
    }

    fn new_state_from_json(&self, json_str: &str) -> Result<Box<super::State>, failure::Error> {
        let state: State = serde_json::from_str(json_str)?;
        Ok(Box::new(state))
    }
}

impl State {
    fn start_ball(&mut self) {
        let (w,h) = screen::GAME_SIZE;
        let w = w as f64;
        let h = h as f64;
        // four options: left-> <-center center-> <-right 
        let angles: [f64; 4] = [30.0, 30.0, 150.0, 150.0];
        let positions = [0.1*w, 0.5*w, 0.5*w, 0.9*w];

        let index = (self.rand.next_u32() % 4) as usize;

        self.ball.position.x = positions[index];
        self.ball.position.y = h / 2.0;
        self.ball.velocity = Vec2D::from_polar(self.config.ball_speed_slow, angles[index].to_radians());
    }
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
    fn keep_paddle_on_screen(&mut self) {
        let left = screen::BOARD_LEFT_X as f64 - self.paddle_width/2.0;
        let right = screen::BOARD_RIGHT_X as f64 + self.paddle_width/2.0;
        if self.paddle.position.x < left {
            self.paddle.position.x = left;
            self.paddle.velocity.x = 0.0;
        } else if self.paddle.position.x > right {
            self.paddle.position.x = right;
            self.paddle.velocity.x = 0.0;
        }
    }
    fn update_time_slice(&mut self, time_step: f64) {
        // Update positions.
        self.ball.integrate_mut(time_step);
        self.paddle.integrate_mut(time_step);
        
        self.keep_paddle_on_screen();

        // Handle collisions:
        if self.ball.velocity.y > 0.0 {
            // check paddle:
            let paddle_ball_same_x = (self.ball.position.x - self.paddle.position.x).abs()
                < self.ball_radius + self.paddle_width / 2.0;
            let paddle_ball_same_y =
                (self.paddle.position.y - self.ball.position.y).abs() < self.ball_radius;
            if paddle_ball_same_x && paddle_ball_same_y {
                // get x location of ball hit relative to paddle
                let ball_hit_x = self.ball.position.x - (self.paddle.position.x - (self.paddle_width/ 2.0));
                // get normalized location of ball hit along paddle
                let paddle_normalized_relative_intersect_x =  1.0 - ball_hit_x / self.paddle_width;
                // convert this normalized parameter to the degree of the bounce angle
                let bounce_angle = paddle_normalized_relative_intersect_x * screen::BALL_ANGLE_RANGE + screen::BALL_ANGLE_MIN;

                self.ball.velocity = Vec2D::from_polar(self.ball.velocity.magnitude(), bounce_angle.to_radians());
                // calculations use non-graphics polar orientation
                // to quickly fix, we reflect over the x-axis
                self.ball.velocity.y *= -1.0;
            }

            // check lose?
            if self.ball.position.y + self.ball_radius > screen::BOARD_BOTTOM_Y.into() {
                if !self.is_dead {
                    self.lives-=1;
                    self.paddle_width = screen::PADDLE_START_SIZE.0.into();
                }
                self.is_dead = true;
                return;
            }
        } else {
            // bounce ceiling?
            if self.ball.position.y - self.ball_radius < screen::BOARD_TOP_Y.into() {
                self.ball.velocity.y *= -1.0;
                self.paddle_width = screen::PADDLE_SMALL_SIZE.0.into();
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
                if brick.depth >= self.config.ball_speed_row_depth {
                    // Potentially speed up the ball. This will be a no-op if it's already fast.
                    let theta = self.ball.velocity.angle();
                    self.ball.velocity = Vec2D::from_polar(self.config.ball_speed_fast, theta);
                }
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
    fn lives(&self) -> i32 {
        self.lives
    }
    fn score(&self) -> i32 {
        self.points
    }

    /// Mutably update the game state.
    fn update_mut(&mut self, buttons: Input) {
        self.update_paddle_movement(buttons);

        if self.is_dead {
            if (buttons.button1 || buttons.button2) {
                self.start_ball();
                self.is_dead = false;
            }
        }

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
                if self.is_dead {
                    // Don't simulate if dead.
                    return;
                }
                time_simulated += time_step;
            }
        }
    }

    fn draw(&self) -> Vec<Drawable> {
        let mut output = Vec::new();
        output.push(Drawable::rect(
            self.config.bg_color,
            0,
            0,
            screen::GAME_SIZE.0,
            screen::GAME_SIZE.1,
        ));

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

        if self.lives < 0 {
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
            self.config.paddle_color,
            paddle_x - paddle_w / 2,
            paddle_y,
            paddle_w,
            screen::PADDLE_START_SIZE.1,
        ));

        let (ball_x, ball_y) = self.ball.position.pixels();
        let ball_r = self.ball_radius as i32;
        output.push(Drawable::rect(
            self.config.ball_color,
            ball_x - ball_r,
            ball_y - ball_r,
            ball_r * 2,
            ball_r * 2,
        ));

        // Draw points:
        output.extend(draw_score(self.points, screen::BOARD_RIGHT_X, 5));
        // Draw lives:
        output.extend(draw_score(self.lives, screen::BOARD_LEFT_X + 50, 5));

        output
    }
    
    fn to_json(&self) -> String {
        serde_json::to_string(self).expect("Should be no JSON Serialization Errors.")
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn test_colors_unique_in_gray() {
        let config = Config::default();
        let num_colors = config.unique_colors().len();
        let uniq_grays: HashSet<u8> = config.unique_colors().into_iter().map(|c| c.grayscale_byte()).collect();
        // Don't allow a grayscale agent to be confused where a human wouldn't be.
        assert_eq!(uniq_grays.len(), num_colors);
    }

}
