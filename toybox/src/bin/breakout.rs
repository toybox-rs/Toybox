extern crate toybox;

extern crate quicksilver;
use quicksilver::{
    State, run,
    geom::{Rectangle, Circle},
    graphics::{Color,Draw,Window,WindowBuilder}
};
use toybox::breakout::*;

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