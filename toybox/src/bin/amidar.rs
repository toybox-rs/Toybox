extern crate toybox;

extern crate quicksilver;
use quicksilver::{
    geom::Rectangle,
    graphics::{Color, Draw, Window, WindowBuilder},
    input::Key,
    run, State,
};
use toybox::amidar;
use toybox::Input;

struct Game {
    state: amidar::State,
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

impl State for Game {
    fn new() -> Game {
        match amidar::State::new() {
            Err(e) => {
                panic!("{:?}", e);
            }
            Ok(state) => Game { state },
        }
    }
    fn update(&mut self, window: &mut Window) {
        let buttons = process_keys(window);
        if self.state.game_over {
            // Any key starts a new game.
            if !buttons.is_empty() {
                self.state = amidar::State::new().expect("Expected creation of new game state ok.");
            }
            return;
        }
        self.state.update_mut(&buttons);
    }
    fn draw(&mut self, window: &mut Window) {
        window.clear(Color::black());

        let track_color = Color::from(RGBA::rgb(148, 0, 211));
        let player_color = Color::from(RGBA::rgb(255, 255, 153));
        let enemy_color = Color::from(RGBA::rgb(255, 255, 153));
        let text_color = player_color.clone();

        if self.state.game_over {
            window.present();
            return;
        }

        let (tile_w, tile_h) = amidar::TILE_SIZE;
        let (offset_x, offset_y) = amidar::BOARD_OFFSET;
        let (board_w, board_h) = self.state.board_size().pixels();

        for (ty, row) in self.state.board.iter().enumerate() {
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

        let (player_x, player_y) = self.state.player.pixels();
        let (entity_w, entity_h) = amidar::ENTITY_SIZE;
        window.draw(
            &Draw::rectangle(Rectangle::new(
                offset_x + player_x - 1,
                offset_y + player_y - 1,
                entity_w,
                entity_h,
            )).with_color(player_color),
        );

        window.present();
    }
}
fn main() {
    let (w, h) = amidar::GAME_SIZE;
    run::<Game>(WindowBuilder::new("Amidar", w as u32, h as u32));
}
