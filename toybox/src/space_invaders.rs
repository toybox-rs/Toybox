use super::graphics::{Color, Drawable, SpriteData};
use super::{Input, Direction};
use failure::Error;

pub mod screen {
    pub const GAME_SIZE: (i32, i32) = (480,319);
    pub const SKY_TO_GROUND: i32 = 298;

    pub const GAME_DOT_LEFT: i32 = 99;
    pub const GAME_DOT_RIGHT: i32 = 108;
    pub const GAME_DOT_SIZE: (i32,i32) = (6,7);
    pub const SHIP_SIZE: (i32,i32) = (21,15);
    pub const SHIELD_SIZE: (i32,i32) = (24,27);
    pub const SHIELD1_POS: (i32,i32) = (126,241);
    pub const SHIELD2_POS: (i32,i32) = (222,241);
    pub const SHIELD3_POS: (i32,i32) = (318,241);

    pub const ENEMY_SIZE: (i32,i32) = (24,15);
    pub const ENEMY_Y_SPACE: i32 = 12;
    pub const ENEMY_X_SPACE: i32 = 24;
    pub const UFO_SIZE: (i32,i32) = (21,13);
    pub const LASER_SIZE: (i32,i32) = (3,11);

    // Colors:
    pub const LEFT_GAME_DOT_COLOR: (u8,u8,u8) = (64,124,64);
    pub const RIGHT_GAME_DOT_COLOR: (u8,u8,u8) = (160,132,68);
    pub const SHIELD_COLOR: (u8,u8,u8) = (172,80,48);
    pub const ENEMY_COLOR: (u8,u8,u8) = (132,132,36);
    pub const UFO_COLOR: (u8,u8,u8) = (140,32,116);
    pub const LASER_COLOR: (u8,u8,u8) = (144,144,144);
    pub const GROUND_COLOR: (u8,u8,u8) = (76,80,28);
    pub const SHIP_COLOR: (u8,u8,u8) = (35,129,59);

    pub const SHIP_LIMIT_X1: i32 = GAME_DOT_LEFT + SHIP_SIZE.0/2;
    pub const SHIP_LIMIT_X2: i32 = GAME_SIZE.0-GAME_DOT_RIGHT - SHIP_SIZE.0/2;

    pub const SHIELD_SPRITE_DATA: &'static str = include_str!("resources/space_invader_shield_x3");
}

pub fn load_sprite(data: &str, on_color: &Color, on_symbol: char, off_symbol: char) -> Result<SpriteData, Error> {
    let off_color = Color::invisible();
    let mut pixels = Vec::new();
    for line in data.lines() {
        let mut pixel_row = Vec::new();
        for ch in line.chars() {
            if ch == on_symbol {
                pixel_row.push(on_color.clone());
            } else if ch == off_symbol {
                pixel_row.push(off_color.clone());
            } else {
                return Err(format_err!("Cannot construct pixel from {}, expected one of (on={}, off={})", ch, on_symbol, off_symbol));
            }
        }
        pixels.push(pixel_row);
    }
    let width = pixels[0].len();
    debug_assert!(pixels.iter().all(|row| row.len() == width));
    Ok(SpriteData::new(pixels, 3))
}
pub fn load_sprite_default(data: &str, on_color: &Color) -> Result<SpriteData, Error> {
    load_sprite(data, on_color, 'X', '.')
}

lazy_static! {
    static ref SHIELD_SPRITE: SpriteData = load_sprite_default(screen::SHIELD_SPRITE_DATA, &(&screen::SHIELD_COLOR).into()).expect("Shield sprite should be included!");
}

#[derive(Debug,Clone,PartialEq,Eq)]
pub struct Actor {
    pub x: i32,
    pub y: i32,
    pub w: i32,
    pub h: i32,
    /// Lasers have a direction.
    pub movement: Option<Direction>,
    /// Many things may have a speed.
    pub speed: i32,
    pub color: Color,
}

impl Default for Actor {
    fn default() -> Self {
        Actor { x: 0, y: 0, w: 0, h: 0, movement: None, speed: 0, color: Color::white() }
    }
}

impl Actor {
    fn ship(x: i32, y: i32) -> Actor {
        let (w, h) = screen::SHIP_SIZE;
        Actor { x, y, w, h, color: (&screen::SHIP_COLOR).into(), ..Default::default() }
    }
    fn enemy(x: i32, y: i32) -> Actor {
        let (w, h) = screen::ENEMY_SIZE;
        Actor { x, y, w, h, color: (&screen::ENEMY_COLOR).into(), ..Default::default() }
    }
    fn laser(x: i32, y: i32, dir: Direction) -> Actor {
        let (w, h) = screen::LASER_SIZE;
        Actor { x, y, w, h, color: (&screen::LASER_COLOR).into(), ..Default::default() }
    }
}

#[derive(Clone)]
pub struct State {
    pub game_over: bool,
    /// Ship is a rectangular actor (logically).
    pub ship: Actor,
    /// Emulate the fact that Atari could only have one laser at a time (and it "recharges" faster if you hit the front row...)
    pub ship_laser: Option<Actor>,
    /// Shields are destructible, so we need to track their pixels...
    pub shields: Vec<SpriteData>,
    /// Enemies are rectangular actors (logically speaking).
    pub enemies: Vec<Actor>,
    /// Enemy lasers are actors as well.
    pub enemy_lasers: Vec<Actor>
}

pub struct SpaceInvaders;
impl super::Simulation for SpaceInvaders {
    fn game_size(&self) -> (i32,i32) {
        screen::GAME_SIZE
    }
    fn new_game(&self) -> Box<super::State> {
        let player_start_x = screen::SHIP_LIMIT_X1;
        let player_start_y = screen::SKY_TO_GROUND - screen::SHIP_SIZE.1;
        let mut shields = Vec::new();

        for (x,y) in &[screen::SHIELD1_POS, screen::SHIELD2_POS, screen::SHIELD3_POS] {
            shields.push(SHIELD_SPRITE.translate(*x, *y))
        }
        Box::new(State {
            game_over: false,
            ship: Actor::ship(player_start_x, player_start_y),
            ship_laser: None,
            shields,
            enemies: Vec::new(),
            enemy_lasers: Vec::new(),
        })
    }
}

impl super::State for State {
    fn game_over(&self) -> bool {
        self.game_over
    }
    fn update_mut(&mut self, buttons: &Input) {

    }
    fn draw(&self) -> Vec<Drawable> {
        let mut output = Vec::new();

        output
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_shield_sprite_size() {
        let sprite = SHIELD_SPRITE.clone();
        assert_eq!(screen::SHIELD_SIZE.0, sprite.width() * sprite.scale());
        assert_eq!(screen::SHIELD_SIZE.1, sprite.height() * sprite.scale());
    }

    #[test]
    pub fn test_create_new_state() {
        let state = State::new();
        assert_eq!(None, state.ship_laser);
    }

}