extern crate toybox;

extern crate quicksilver;
use quicksilver::{
    geom::{Circle, Rectangle},
    graphics::{Color, Draw, Window, WindowBuilder},
    input::Key,
    run, State,
};
use toybox::breakout::*;
use toybox::Input;

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
        _ => Color::white(),
    }
}

fn process_keys(window: &Window) -> Vec<Input> {
    let keys = window.keyboard();
    let mut buttons = Vec::new();

    if keys[Key::Up].is_down() || keys[Key::W].is_down() {
        buttons.push(Input::Up);
    }
    if keys[Key::Down].is_down() || keys[Key::S].is_down() {
        buttons.push(Input::Down);
    }
    if keys[Key::Left].is_down() || keys[Key::A].is_down() {
        buttons.push(Input::Left);
    }
    if keys[Key::Right].is_down() || keys[Key::D].is_down() {
        buttons.push(Input::Right);
    }
    if keys[Key::Z].is_down() || keys[Key::Space].is_down() {
        buttons.push(Input::Button1);
    }
    if keys[Key::X].is_down() {
        buttons.push(Input::Button2);
    }

    buttons
}

impl State for BreakoutGame {
    fn new() -> BreakoutGame {
        BreakoutGame {
            state: BreakoutState::new(),
        }
    }
    fn draw(&mut self, window: &mut Window) {
        window.clear(Color::black());

        let buttons = process_keys(window);

        if self.state.game_over {
            window.present();

            // Any key starts a new game.
            if !buttons.is_empty() {
                self.state = BreakoutState::new();
            }
            return;
        }

        for brick in self.state.bricks.iter().filter(|b| b.alive) {
            let (x, y) = brick.position.pixels();
            let (w, h) = brick.size.pixels();
            window.draw(
                &Draw::rectangle(Rectangle::new(x, y, w - 1, h - 1))
                    .with_color(score_to_color(brick.points)),
            );
        }

        let (paddle_x, paddle_y) = self.state.paddle.position.pixels();
        let paddle_w = self.state.paddle_width as i32;

        window.draw(
            &Draw::rectangle(Rectangle::new(
                paddle_x - paddle_w / 2,
                paddle_y,
                paddle_w,
                10,
            )).with_color(Color::indigo()),
        );
        let (ball_x, ball_y) = self.state.ball.position.pixels();
        let ball_r = self.state.ball_radius as i32;

        window.draw(&Draw::circle(Circle::new(ball_x, ball_y, ball_r)).with_color(Color::white()));

        self.state.update_mut(1.0, &buttons);

        window.present();
    }
}
fn main() {
    let (w, h) = GAME_SIZE;
    run::<BreakoutGame>(WindowBuilder::new("Breakout", w as u32, h as u32));
}
