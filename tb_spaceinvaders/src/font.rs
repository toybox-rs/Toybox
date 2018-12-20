use toybox_core::graphics::{load_digit_sprites, Color, Drawable, FixedSpriteData};

const SET: char = 'X';
const IGNORE: char = '.';
pub const DIGIT_WIDTH: i32 = 24;
pub const DIGIT_HEIGHT: i32 = 9;
pub const DIGIT_PAD: i32 = 8;

pub const LEFT_GAME_DOT_COLOR: (u8, u8, u8) = (64, 124, 64);
pub const RIGHT_GAME_DOT_COLOR: (u8, u8, u8) = (160, 132, 68);

lazy_static! {
    static ref LEFT_DIGIT_SPRITES: Vec<FixedSpriteData> = load_digit_sprites(
        include_str!("resources/space_invaders_digit_sprites.txt"),
        Color::rgb(
            LEFT_GAME_DOT_COLOR.0,
            LEFT_GAME_DOT_COLOR.1,
            LEFT_GAME_DOT_COLOR.2
        ),
        SET,
        IGNORE
    );
    static ref RIGHT_DIGIT_SPRITES: Vec<FixedSpriteData> = load_digit_sprites(
        include_str!("resources/space_invaders_digit_sprites.txt"),
        Color::rgb(
            RIGHT_GAME_DOT_COLOR.0,
            RIGHT_GAME_DOT_COLOR.1,
            RIGHT_GAME_DOT_COLOR.2
        ),
        SET,
        IGNORE
    );
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Debug)]
pub enum Side {
    LEFT,
    RIGHT,
}

/// Given a font return an atomic reference to a digit sprite as a FixedSpriteData.
fn get_sprite(digit_index: u32, side: Side) -> FixedSpriteData {
    debug_assert!(digit_index < 10);
    // Space invaders digits in the text file were entered reverse from the other digits, which
    // is why we are doing 9 - digit_index here.
    match side {
        Side::LEFT => LEFT_DIGIT_SPRITES[9 - digit_index as usize].clone(),
        Side::RIGHT => RIGHT_DIGIT_SPRITES[9 - digit_index as usize].clone(),
    }
}

pub fn draw_score(score: i32, x: i32, y: i32, side: Side) -> Vec<Drawable> {
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
