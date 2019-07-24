use spaceinvaders::screen::{LEFT_GAME_DOT_COLOR, LIVES_DISPLAY_COLOR, RIGHT_GAME_DOT_COLOR};
use toybox_core::graphics::{load_digit_sprites, Drawable, FixedSpriteData};

const SET: char = 'X';
const IGNORE: char = '.';
pub const DIGIT_WIDTH: i32 = 24;
pub const DIGIT_PAD: i32 = 8;
pub const _DIGIT_HEIGHT: i32 = 9;
pub const _LIVES_DIGIT_HEIGHT: i32 = 10;

lazy_static! {
    static ref LEFT_DIGIT_SPRITES: Vec<FixedSpriteData> = load_digit_sprites(
        include_str!("resources/space_invaders_digit_sprites.txt"),
        (&LEFT_GAME_DOT_COLOR).into(),
        SET,
        IGNORE
    );
    static ref RIGHT_DIGIT_SPRITES: Vec<FixedSpriteData> = load_digit_sprites(
        include_str!("resources/space_invaders_digit_sprites.txt"),
        (&RIGHT_GAME_DOT_COLOR).into(),
        SET,
        IGNORE
    );
    static ref LIVES_DIGIT_SPRITES: Vec<FixedSpriteData> = load_digit_sprites(
        include_str!("resources/lives_font.txt"),
        (&LIVES_DISPLAY_COLOR).into(),
        SET,
        IGNORE
    );
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Debug)]
pub enum FontChoice {
    LEFT,
    RIGHT,
    LIVES,
}

/// Given a font return an atomic reference to a digit sprite as a FixedSpriteData.
pub fn get_sprite(digit_index: u32, side: FontChoice) -> FixedSpriteData {
    debug_assert!(digit_index < 10);
    // Space invaders digits in the text file were entered reverse from the other digits, which
    // is why we are doing 9 - digit_index here.
    match side {
        FontChoice::LEFT => LEFT_DIGIT_SPRITES[9 - digit_index as usize].clone(),
        FontChoice::RIGHT => RIGHT_DIGIT_SPRITES[9 - digit_index as usize].clone(),
        FontChoice::LIVES => LIVES_DIGIT_SPRITES[9 - digit_index as usize].clone(),
    }
}

pub fn draw_score(score: i32, x: i32, y: i32, side: FontChoice) -> Vec<Drawable> {
    let score = if score < 0 { 0 } else { score as u32 };
    let width = DIGIT_WIDTH;
    let pad = DIGIT_PAD;
    let radix = 10;
    // Outer digits are 1px higher than inner ones.
    format!("{:04}", score)
        .chars()
        .map(|ch| ch.to_digit(radix).expect("format! only gives us digits!!"))
        .enumerate()
        .map(|(position, digit)| {
            let x = x + (position as i32) * (width + pad);
            let y = match position {
                0 | 3 => y,
                1 | 2 => y + 1,
                _ => panic!("There should only be 4 digits in the score."),
            };
            Drawable::sprite(x, y, get_sprite(digit, side))
        })
        .collect()
}
