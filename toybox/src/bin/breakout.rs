extern crate toybox;

mod breakout {
    use toybox::Vec2D;
    use toybox::Body2D;

    pub const GameSize: (i32,i32) = (240, 240);

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
            let (w,h) = GameSize;
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
                ball: ball,
                ball_radius: 4.0,
                paddle: Body2D::new_pos((w as f64) / 2.0, (h as f64) - 30.0),
                paddle_width: 32.0,
                bricks
            }
        }
        /// Mutably update the game state with a given time-step.
        pub fn update_mut(&mut self, time_step: f64) {
            // Update positions.
            self.ball.integrate_mut(time_step);
            self.paddle.integrate_mut(time_step);

            let game_width = GameSize.0 as f64;
            let game_height = GameSize.1 as f64;
            
            // Handle collisions:
            if self.ball.velocity.y > 0.0 {
                // ball is "falling", check floor, paddle, bricks

                // check lose?
                if (self.ball.position.y + self.ball_radius > game_height) {
                    // TODO, lose
                    self.ball.velocity.y *= -1.0;
                }

            } else {
                // ball is "rising", check ceiling, bricks

                // bounce ceiling?
                if (self.ball.position.y - self.ball_radius < 0.0) {
                    self.ball.velocity.y *= -1.0;
                }
            }

            if (self.ball.velocity.x > 0.0) {
                // bounce right wall?
                if (self.ball.position.x + self.ball_radius > game_width) {
                    self.ball.velocity.x *= -1.0;
                }
            } else {
                // bounce left wall?
                if (self.ball.position.x - self.ball_radius < 0.0) {
                    self.ball.velocity.x *= -1.0;
                }

            }
        }
    }
}

extern crate quicksilver;
use quicksilver::{
    State, run,
    geom::{Rectangle, Circle},
    graphics::{Color,Draw,Window,WindowBuilder}
};
use breakout::*;

struct BreakoutGame {
    state: BreakoutState,
}

fn score_to_color(score: u32) -> Color {
    match score {
        1 => Color::red(),
        2 => Color::orange(),
        3 => Color::yellow(),
        4 => Color::green(),
        5 => Color::blue(),
        _ => Color::white()
    }
}

impl State for BreakoutGame {
    fn new() -> BreakoutGame {
        BreakoutGame { state: BreakoutState::new() }
    }
    fn draw(&mut self, window: &mut Window) {
        window.clear(Color::black());
        for brick in &self.state.bricks {
            let (x,y) = brick.position.pixels();
            let (w,h) = brick.size.pixels();
            window.draw(&Draw::rectangle(Rectangle::new(x,y,w-1,h-1))
                            .with_color(score_to_color(brick.points)));
        }

        let (paddle_x, paddle_y) = self.state.paddle.position.pixels();
        let paddle_w = self.state.paddle_width as i32;

        window.draw(&Draw::rectangle(Rectangle::new(paddle_x - paddle_w/2, paddle_y, paddle_w, 10))
                        .with_color(Color::indigo()));
        let (ball_x, ball_y) = self.state.ball.position.pixels();
        let ball_r = self.state.ball_radius as i32;
        window.draw(&Draw::circle(Circle::new(ball_x, ball_y, ball_r))
                        .with_color(Color::white()));

        self.state.update_mut(1.0);

        window.present();
    }
}
fn main() {
    let (w,h) = GameSize;
    run::<BreakoutGame>(WindowBuilder::new("Breakout", w as u32, h as u32));
}