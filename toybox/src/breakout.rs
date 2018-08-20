use super::vec2d::Vec2D;
use super::Body2D;
use super::Input;

pub const GAME_SIZE: (i32,i32) = (240, 240);

#[derive(Debug, Clone)]
pub struct Brick {
    /// Brick position describes the upper-left of the brick.
    pub position: Vec2D,
    /// Brick size is the width and height of the brick.
    pub size: Vec2D,
    /// This is the number of points for a brick.
    pub points: u32,
    // todo, color?
}

impl Brick {
    pub fn new(position: Vec2D, size: Vec2D, points: u32) -> Brick {
        Brick { position, size, points }
    }
}

#[derive(Debug, Clone)]
pub struct BreakoutState {
    pub game_over: bool,
    /// ball position describes the center of the ball.
    pub ball: Body2D,
    pub ball_radius: f64,
    /// paddle position describes the center of the paddle.
    pub paddle: Body2D,
    pub paddle_width: f64,
    pub bricks: Vec<Brick>,
}

impl BreakoutState {
    /// Create a new game of breakout.
    pub fn new() -> BreakoutState {
        let (w,h) = GAME_SIZE;
        let mut bricks = Vec::new();
        let mut ball = Body2D::new_pos((w as f64) / 2.0, (h as f64) / 2.0);
        ball.velocity = Vec2D::from_polar(4.0, 0.5);

        let num_bricks_across = 12;
        let num_bricks_deep = 5;
        let xs = (w as f64) / (num_bricks_across as f64);
        let ys = 12.0;
        let bsize = Vec2D::new(xs, ys);
        for x in 0..num_bricks_across {
            for y in 0..num_bricks_deep {
                let bpos = Vec2D::new(x as f64 * xs, y as f64 * ys) ;
                bricks.push(Brick::new(bpos, bsize.clone(), y+1));
            }
        }

        BreakoutState { 
            game_over: false,
            ball: ball,
            ball_radius: 4.0,
            paddle: Body2D::new_pos((w as f64) / 2.0, (h as f64) - 30.0),
            paddle_width: 32.0,
            bricks
        }
    }
    
    /// Mutably update the game state with a given time-step.
    pub fn update_mut(&mut self, time_step: f64, buttons: &[Input]) {
        // Update positions.
        self.ball.integrate_mut(time_step);
        self.paddle.integrate_mut(time_step);

        let game_width = GAME_SIZE.0 as f64;
        let game_height = GAME_SIZE.1 as f64;

        let left = buttons.contains(&Input::Left);
        let right = buttons.contains(&Input::Right);

        if left {
            self.paddle.velocity.x = -3.0;
        } else if right {
            self.paddle.velocity.x = 3.0;
        } else {
            self.paddle.velocity.x = 0.0;
        }
        
        // Handle collisions:
        if self.ball.velocity.y > 0.0 {
            // ball is "falling", check floor, paddle, bricks

            // check lose?
            if self.ball.position.y + self.ball_radius > game_height {
                // TODO, lose
                self.game_over = true;
                eprintln!("Press any key, e.g., SPACE to reset the game!");
            }

        } else {
            // ball is "rising", check ceiling, bricks

            // bounce ceiling?
            if self.ball.position.y - self.ball_radius < 0.0 {
                self.ball.velocity.y *= -1.0;
            }
        }

        if self.ball.velocity.x > 0.0 {
            // bounce right wall?
            if self.ball.position.x + self.ball_radius > game_width {
                self.ball.velocity.x *= -1.0;
            }
        } else {
            // bounce left wall?
            if self.ball.position.x - self.ball_radius < 0.0 {
                self.ball.velocity.x *= -1.0;
            }

        }
    }
}