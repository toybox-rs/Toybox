extern crate toybox;
extern crate failure;

use failure::Error;

extern crate quicksilver;
use quicksilver::{
    geom::{Rectangle, Vector},
    Future,
    graphics::{Color, Draw, View, Window, WindowBuilder, Font},
    input::Key,
    run, State,
};
use toybox::amidar;
use toybox::Input;

struct Game {
    state: amidar::State,
    font: Font,
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

#[derive(Clone, Copy, Debug)]
struct RGBA {
    r: u8,
    g: u8,
    b: u8,
    a: u8,
}

impl RGBA {
    fn rgb(r: u8, g: u8, b: u8) -> RGBA {
        RGBA { r, g, b, a: 255 }
    }
}

impl From<RGBA> for Color {
    fn from(c: RGBA) -> Color {
        Color {
            r: f32::from(c.r) / 255.0,
            g: f32::from(c.g) / 255.0,
            b: f32::from(c.b) / 255.0,
            a: f32::from(c.a) / 255.0,
        }
    }
}

fn try_setup_game() -> Result<Game, Error> {
    let state = amidar::State::try_new()?;
    let font = Font::load("src/resources/arcadeclassic.regular.ttf").wait()?;
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
        let buttons = process_keys(window);
        if self.state.game_over {
            // Any key starts a new game.
            if !buttons.is_empty() {
                self.state =
                    amidar::State::try_new().expect("Expected creation of new game state ok.");
            }
            return;
        }
        // terrible hack to get to 30FPS instead of 60FPS with this game framework.
        self.state.update_mut(&buttons);
    }
    fn draw(&mut self, window: &mut Window) {
        let (w, h) = amidar::screen::GAME_SIZE;
        window.set_view(View::new(Rectangle::new(0, 0, w, h)));
        window.clear(Color::black());

        let track_color = Color::from(RGBA::rgb(148, 0, 211));
        let player_color = Color::from(RGBA::rgb(255, 255, 153));
        let enemy_color = Color::from(RGBA::rgb(255, 0, 153));
        let text_color = player_color.clone();

        if self.state.game_over {
            window.present();
            return;
        }

        let (tile_w, tile_h) = amidar::screen::TILE_SIZE;
        let (offset_x, offset_y) = amidar::screen::BOARD_OFFSET;

        for (ty, row) in self.state.board.tiles.iter().enumerate() {
            let ty = ty as i32;
            for (tx, tile) in row.iter().enumerate() {
                let tx = tx as i32;
                let tile_color = match tile {
                    // TODO: change this color:
                    amidar::Tile::Painted => Color::white(),
                    amidar::Tile::Unpainted => track_color,
                    amidar::Tile::Empty => continue,
                };
                window.draw(
                    &Draw::rectangle(Rectangle::new(
                        offset_x + tx * tile_w,
                        offset_y + ty * tile_h,
                        tile_w,
                        tile_h,
                    )).with_color(tile_color),
                );
            }
        }

        let (player_x, player_y) = self.state.player.position.to_screen().pixels();
        let (player_w, player_h) = amidar::screen::PLAYER_SIZE;
        window.draw(
            &Draw::rectangle(Rectangle::new(
                offset_x + player_x - 1,
                offset_y + player_y - 1,
                player_w,
                player_h,
            )).with_color(player_color),
        );

        for enemy in self.state.enemies.iter() {
            let (x, y) = enemy.position.to_screen().pixels();
            let (w, h) = amidar::screen::ENEMY_SIZE;

            window.draw(
                &Draw::rectangle(Rectangle::new(offset_x + x - 1, offset_y + y - 1, w, h))
                    .with_color(enemy_color),
            );
        }

        // Draw score:
        let (points_x, points_y) = (104, 198);
        let score_img = self.font.render(&format!("{}", self.state.score), 16.0, text_color);
        let score_width = score_img.area().width as i32;
        window.draw(&Draw::image(&score_img, Vector::new(points_x-score_width, points_y)));

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
