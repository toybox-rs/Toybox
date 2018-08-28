extern crate failure;
extern crate toybox;
extern crate human_play;
use toybox::graphics::Drawable;

use failure::Error;

extern crate quicksilver;
use quicksilver::{
    geom::{Rectangle, Vector},
    graphics::{Color, Draw, Font, View, Window, WindowBuilder},
    input::Key,
    run, Future, State,
};
use toybox::amidar;
use toybox::Input;

struct Game {
    state: amidar::State,
    font: Font,
}

fn try_setup_game() -> Result<Game, Error> {
    let state = amidar::State::try_new()?;
    let font = Font::load("src/resources/PressStart2P.ttf").wait()?;
    Ok(Game { state, font })
}

impl State for Game {
    fn new() -> Game {
        match try_setup_game() {
            Err(e) => {
                panic!("{:?}", e);
            }
            Ok(game) => game,
        }
    }
    fn update(&mut self, window: &mut Window) {
        let buttons = human_play::process_keys(window);
        if self.state.game_over {
            // Any key starts a new game.
            if !buttons.is_empty() {
                self.state =
                    amidar::State::try_new().expect("Expected creation of new game state ok.");
            }
            return;
        }
        self.state.update_mut(&buttons);
    }
    fn draw(&mut self, window: &mut Window) {
        let (w, h) = amidar::screen::GAME_SIZE;
        window.set_view(View::new(Rectangle::new(0, 0, w, h)));
        window.clear(Color::black());

        let drawables = self.state.draw();
        for dw in drawables {
            match dw {
                Drawable::Rectangle { color, x, y, w, h } => {
                    window.draw(
                        &Draw::rectangle(Rectangle::new(x, y, w, h)).with_color(Color {
                            r: color.r as f32 / 255.0,
                            g: color.g as f32 / 255.0,
                            b: color.b as f32 / 255.0,
                            a: 1.0,
                        }),
                    );
                }
            }
        }

        let text_color = Color {
            r: 1.0,
            g: 1.0,
            b: 153.0 / 255.0,
            a: 1.0,
        };

        // Draw score:
        let (points_x, points_y) = (104, 198);
        let score_img = self
            .font
            .render(&format!("{}", self.state.score), 8.0, text_color);
        let score_width = score_img.area().width as i32;
        window.draw(&Draw::image(
            &score_img,
            Vector::new(points_x - score_width, points_y),
        ));

        window.present();
    }
}

fn main() {
    let (w, h) = amidar::screen::GAME_SIZE;
    let scale = 3;
    run::<Game>(WindowBuilder::new(
        "Amidar",
        scale * w as u32,
        scale * h as u32,
    ));
}
