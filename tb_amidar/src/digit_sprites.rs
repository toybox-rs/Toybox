use toybox_core::graphics::{load_digit_sprites, Color, Drawable, FixedSpriteData};

const SET: char = '1';
const IGNORE: char = '.';
pub const DIGIT_WIDTH: i32 = 8;
pub const DIGIT_HEIGHT: i32 = 7;

lazy_static! {
    static ref DIGIT_SPRITES: Vec<FixedSpriteData> = load_digit_sprites(
        include_str!("resources/amidar_digit_sprites.txt"),
        Color::rgb(255, 255, 153),
        SET,
        IGNORE
    );
}

/// Given a font return an atomic reference to a digit sprite as a FixedSpriteData.
fn get_sprite(digit_index: u32) -> FixedSpriteData {
    debug_assert!(digit_index < 10);
    DIGIT_SPRITES[digit_index as usize].clone()
}

/// Draw score for Amidar.
pub fn draw_score(lives: i32, x: i32, y: i32) -> Vec<Drawable> {
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
    pub fn test_amidar_font() {
        for digit in 0..10 {
            let sprite = get_sprite(digit);
            assert_eq!(sprite.width(), DIGIT_WIDTH);
            assert_eq!(sprite.height(), DIGIT_HEIGHT);
        }
    }
}
