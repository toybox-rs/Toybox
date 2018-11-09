use super::graphics::{Color, Drawable, SpriteData};
use super::{Direction, Input};
use failure::Error;
use serde_json;
use std::any::Any;

pub mod screen {
    pub const GAME_SIZE: (i32, i32) = (480, 319);
    pub const SKY_TO_GROUND: i32 = 298;

    pub const GAME_DOT_LEFT: i32 = 99;
    pub const GAME_DOT_RIGHT: i32 = 108;
    pub const GAME_DOT_SIZE: (i32, i32) = (6, 7);
    pub const SHIP_SIZE: (i32, i32) = (21, 15);
    pub const SHIELD_SIZE: (i32, i32) = (24, 27);
    pub const SHIELD1_POS: (i32, i32) = (126, 241);
    pub const SHIELD2_POS: (i32, i32) = (222, 241);
    pub const SHIELD3_POS: (i32, i32) = (318, 241);

    pub const ENEMY_SIZE: (i32, i32) = (24, 15);
    pub const ENEMY_Y_SPACE: i32 = 12;
    pub const ENEMY_X_SPACE: i32 = 24;
    pub const UFO_SIZE: (i32, i32) = (21, 13);
    pub const LASER_SIZE: (i32, i32) = (3, 11);

    // Colors:
    pub const LEFT_GAME_DOT_COLOR: (u8, u8, u8) = (64, 124, 64);
    pub const RIGHT_GAME_DOT_COLOR: (u8, u8, u8) = (160, 132, 68);
    pub const SHIELD_COLOR: (u8, u8, u8) = (172, 80, 48);
    pub const ENEMY_COLOR: (u8, u8, u8) = (132, 132, 36);
    pub const UFO_COLOR: (u8, u8, u8) = (140, 32, 116);
    pub const LASER_COLOR: (u8, u8, u8) = (144, 144, 144);
    pub const GROUND_COLOR: (u8, u8, u8) = (76, 80, 28);
    pub const SHIP_COLOR: (u8, u8, u8) = (35, 129, 59);

    pub const SHIP_LIMIT_X1: i32 = GAME_DOT_LEFT + SHIP_SIZE.0 / 2;
    pub const SHIP_LIMIT_X2: i32 = GAME_SIZE.0 - GAME_DOT_RIGHT - SHIP_SIZE.0 / 2;

    pub const SHIELD_SPRITE_DATA: &str = include_str!("resources/space_invader_shield_x3");
}

pub fn load_sprite(
    data: &str,
    on_color: Color,
    on_symbol: char,
    off_symbol: char,
) -> Result<SpriteData, Error> {
    let off_color = Color::invisible();
    let mut pixels = Vec::new();
    for line in data.lines() {
        let mut pixel_row = Vec::new();
        for ch in line.chars() {
            if ch == on_symbol {
                pixel_row.push(on_color);
            } else if ch == off_symbol {
                pixel_row.push(off_color);
            } else {
                return Err(format_err!(
                    "Cannot construct pixel from {}, expected one of (on={}, off={})",
                    ch,
                    on_symbol,
                    off_symbol
                ));
            }
        }
        pixels.push(pixel_row);
    }
    let width = pixels[0].len();
    debug_assert!(pixels.iter().all(|row| row.len() == width));
    Ok(SpriteData::new(pixels, 3))
}
pub fn load_sprite_default(data: &str, on_color: Color) -> Result<SpriteData, Error> {
    load_sprite(data, on_color, 'X', '.')
}

lazy_static! {
    static ref SHIELD_SPRITE: SpriteData =
        load_sprite_default(screen::SHIELD_SPRITE_DATA, (&screen::SHIELD_COLOR).into())
            .expect("Shield sprite should be included!");
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
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
        Actor {
            x: 0,
            y: 0,
            w: 40,
            h: 0,
            movement: None,
            speed: 3,
            color: Color::white(),
        }
    }
}

impl Actor {
    fn ship(x: i32, y: i32) -> Actor {
        let (w, h) = screen::SHIP_SIZE;
        Actor {
            x,
            y,
            w,
            h,
            color: (&screen::SHIP_COLOR).into(),
            ..Default::default()
        }
    }
    fn enemy(x: i32, y: i32) -> Actor {
        let (w, h) = screen::ENEMY_SIZE;
        Actor {
            x,
            y,
            w,
            h,
            color: (&screen::ENEMY_COLOR).into(),
            ..Default::default()
        }
    }
    fn laser(x: i32, y: i32, dir: Direction) -> Actor {
        let (w, h) = screen::LASER_SIZE;
        Actor {
            x,
            y,
            w,
            h,
            color: (&screen::LASER_COLOR).into(),
            movement: Some(dir),
            speed: 5,
        }
    }
    fn update_mut(&mut self) -> bool {
        if let Some(dir) = self.movement {
            let (dx, dy) = dir.delta();
            self.x += dx * self.speed;
            self.y += dy * self.speed;
            true
        } else {
            false
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct State {
    pub lives: i32,
    pub score: i32,
    /// Ship is a rectangular actor (logically).
    pub ship: Actor,
    /// Emulate the fact that Atari could only have one laser at a time (and it "recharges" faster if you hit the front row...)
    pub ship_laser: Option<Actor>,
    /// Shields are destructible, so we need to track their pixels...
    pub shields: Vec<SpriteData>,
    /// Enemies are rectangular actors (logically speaking).
    pub enemies: Vec<Actor>,
    /// Enemy lasers are actors as well.
    pub enemy_lasers: Vec<Actor>,
}

impl State {
    fn new() -> State {
        let player_start_x = screen::SHIP_LIMIT_X1;
        let player_start_y = screen::SKY_TO_GROUND - screen::SHIP_SIZE.1;
        let mut shields = Vec::new();

        for &(x, y) in &[
            screen::SHIELD1_POS,
            screen::SHIELD2_POS,
            screen::SHIELD3_POS,
        ] {
            shields.push(SHIELD_SPRITE.translate(x, y))
        }
        State {
            lives: 0,
            score: 0,
            ship: Actor::ship(player_start_x, player_start_y),
            ship_laser: None,
            shields,
            enemies: Vec::new(),
            enemy_lasers: Vec::new(),
        }
    }
}

pub struct SpaceInvaders;
impl super::Simulation for SpaceInvaders {
    fn as_any(&self) -> &Any {
        self
    }
    fn reset_seed(&mut self, _seed: u32) {}
    fn game_size(&self) -> (i32, i32) {
        screen::GAME_SIZE
    }
    fn new_game(&mut self) -> Box<super::State> {
        Box::new(State::new())
    }
    fn new_state_from_json(&self, json_str: &str) -> Result<Box<super::State>, Error> {
        let state: State = serde_json::from_str(json_str)?;
        Ok(Box::new(state))
    }
    fn new_state_config_from_json(
        &self,
        json_config: &str,
        json_state: &str,
    ) -> Result<Box<super::State>, Error> {
        panic!("No config implemented for SpaceInvaders.")
    }
}

impl super::State for State {
    fn as_any(&self) -> &Any {
        self
    }
    fn lives(&self) -> i32 {
        self.lives
    }
    fn score(&self) -> i32 {
        self.score
    }
    fn update_mut(&mut self, buttons: Input) {
        self.ship.movement = if buttons.left {
            Some(Direction::Left)
        } else if buttons.right {
            Some(Direction::Right)
        } else {
            None
        };

        if self.ship.update_mut() {
            if self.ship.x > screen::SHIP_LIMIT_X2 {
                self.ship.x = screen::SHIP_LIMIT_X2;
            } else if self.ship.x < screen::SHIP_LIMIT_X1 {
                self.ship.x = screen::SHIP_LIMIT_X1;
            }
        }
        // Only shoot a laser if not present:
        if self.ship_laser.is_none() && buttons.button1 {
            self.ship_laser = Some(Actor::laser(
                self.ship.x + self.ship.w / 2,
                self.ship.y,
                Direction::Up,
            ));
        }
        let delete_laser = if let &mut Some(ref mut laser) = &mut self.ship_laser {
            laser.update_mut() && laser.y < 0
        } else {
            false
        };
        if delete_laser {
            self.ship_laser = None;
        }
    }
    fn draw(&self) -> Vec<Drawable> {
        let mut output = Vec::new();
        output.push(Drawable::rect(
            Color::black(),
            0,
            0,
            screen::GAME_SIZE.0,
            screen::GAME_SIZE.1,
        ));
        // draw ground:
        output.push(Drawable::rect(
            (&screen::GROUND_COLOR).into(),
            0,
            screen::SKY_TO_GROUND,
            screen::GAME_SIZE.0,
            screen::GAME_SIZE.1 - screen::SKY_TO_GROUND,
        ));

        if self.lives() < 0 {
            return output;
        }

        output.push(Drawable::rect(
            self.ship.color,
            self.ship.x,
            self.ship.y,
            self.ship.w,
            self.ship.h,
        ));

        for shield in &self.shields {
            output.push(Drawable::DestructibleSprite(shield.clone()));
        }

        if let Some(ref laser) = self.ship_laser {
            output.push(Drawable::rect(
                laser.color,
                laser.x,
                laser.y,
                laser.w,
                laser.h,
            ))
        }

        output
    }

    fn to_json(&self) -> String {
        serde_json::to_string(self).expect("Should be no JSON Serialization Errors.")
    }

    fn config_to_json(&self) -> String {
        panic!("No config implemented for SpaceInvaders.")
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
