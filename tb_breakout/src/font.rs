use toybox_core::graphics::{load_digit_sprites, Color, Drawable, FixedSpriteData};

/// The breakout font is represented with this SET character for filled in pixels.
const SET: char = '1';
/// The breakout font is represented with this IGNORE character for transparent pixels.
const IGNORE: char = '.';
/// Each digit in the breakout font has this width.
pub const DIGIT_WIDTH: i32 = 24;
/// Each digit in the breakout font has this height.
#[cfg(test)]
pub const DIGIT_HEIGHT: i32 = 12;

lazy_static! {
    /// The score display in Breakout uses this font.
    static ref DIGIT_SPRITES: Vec<FixedSpriteData> = load_digit_sprites(
        include_str!("resources/digit_sprites.txt"),
        Color::rgb(144, 144, 144),
        SET,
        IGNORE
    );
}

/// Given a font return an atomic reference to a digit sprite as a FixedSpriteData.
fn get_sprite(digit_index: u32) -> FixedSpriteData {
    debug_assert!(digit_index < 10);
    DIGIT_SPRITES[digit_index as usize].clone()
}

/// We don't have a sprite for negatives, but scores might be someday... this just prints zero.
/// x and y represent the top-right of scores.
pub fn draw_score(score: i32, x: i32, y: i32) -> Vec<Drawable> {
    let score = if score < 0 { 0 } else { score as u32 };
    let width = DIGIT_WIDTH;
    let radix = 10;
    format!("{:03}", score)
        .chars()
        .map(|ch| ch.to_digit(radix).expect("format! only gives us digits!"))
        .rev()
        .enumerate()
        .map(|(position, digit)| {
            let x = x - (position as i32) * width;
            Drawable::sprite(x, y, get_sprite(digit))
        })
        .collect()
}

/// This is separate from draw_score because in breakout, lives are not
/// padded, but score is, and Rust requires the format string be a literal
pub fn draw_lives(lives: i32, x: i32, y: i32) -> Vec<Drawable> {
    let radix = 10;
    let width = DIGIT_WIDTH;
    format!("{}", lives)
        .chars()
        .map(|ch| ch.to_digit(radix).expect("format! only gives us digits!"))
        .rev()
        .enumerate()
        .map(|(position, digit)| {
            let x = x - (position as i32) * width;
            Drawable::sprite(x, y, get_sprite(digit))
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_breakout_font() {
        for digit in 0..10 {
            let sprite = get_sprite(digit);
            assert_eq!(sprite.width(), DIGIT_WIDTH);
            assert_eq!(sprite.height(), DIGIT_HEIGHT);
        }
    }
}
