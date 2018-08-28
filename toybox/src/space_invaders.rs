use super::graphics::{Color, Drawable};
use super::Input;
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
    pub const BULLET_SIZE: (i32,i32) = (3,11);

    // Colors:
    pub const LEFT_GAME_DOT_COLOR: (u8,u8,u8) = (64,124,64);
    pub const RIGHT_GAME_DOT_COLOR: (u8,u8,u8) = (160,132,68);
    pub const SHIELD_COLOR: (u8,u8,u8) = (172,80,48);
    pub const ENEMY_COLOR: (u8,u8,u8) = (132,132,36);
    pub const UFO_COLOR: (u8,u8,u8) = (140,32,116);
    pub const LASER_COLOR: (u8,u8,u8) = (144,144,144);
    pub const GROUND_COLOR: (u8,u8,u8) = (76,80,28);

    pub const SHIP_LIMIT_X1: i32 = GAME_DOT_LEFT + SHIP_SIZE.0/2;
    pub const SHIP_LIMIT_X2: i32 = GAME_SIZE.0-GAME_DOT_RIGHT - SHIP_SIZE.0/2;

    pub const SHIELD_SPRITE_DATA: &'static str = include_str!("resources/space_invader_shield_x3");
}

pub fn load_sprite(data: &str, on_color: &Color, on_symbol: char, off_symbol: char) -> Result<Drawable, Error> {
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
    Ok(Drawable::Sprite { data: pixels, scale: 3 })
}
pub fn load_sprite_default(data: &str, on_color: &Color) -> Result<Drawable, Error> {
    load_sprite(data, on_color, 'X', '.')
}

lazy_static! {
    static ref SHIELD_SPRITE: Drawable = load_sprite_default(screen::SHIELD_SPRITE_DATA, &(&screen::SHIELD_COLOR).into()).expect("Shield sprite should be included!");
}

pub struct State {

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_shield_sprite_size() {
        let (sprite, scale) = match SHIELD_SPRITE.clone() {
            Drawable::Sprite { data, scale } => (data, scale),
            _ => panic!(),
        };
        assert_eq!(screen::SHIELD_SIZE.1, (sprite.len() as i32) * scale);
        assert_eq!(screen::SHIELD_SIZE.0, (sprite[0].len() as i32) * scale);
    }

}