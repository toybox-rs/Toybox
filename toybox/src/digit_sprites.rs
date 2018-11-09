use super::graphics::{Color, Drawable, FixedSpriteData};

const RAW_NUMBER_DATA: &str = include_str!("resources/number_sprites.txt");
const SET: char = '1';
const IGNORE: char = '.';
pub const DIGIT_WIDTH: i32 = 24;
pub const DIGIT_HEIGHT: i32 = 12;

lazy_static! {
    static ref DIGIT_SPRITES: Vec<FixedSpriteData> = load_digit_sprites();
}

/// Returns a copy for now...
fn get_sprite(digit_index: u32) -> FixedSpriteData {
    debug_assert!(digit_index < 10);
    DIGIT_SPRITES[digit_index as usize].clone()
}

/// We don't have a sprite for negatives, but scores might be someday... this just prints zero.
/// x and y represent the top-right of scores.
pub fn draw_score(score: i32, x: i32, y: i32) -> Vec<Drawable> {
    let score = if score < 0 { 0 } else { score as u32 };
    let radix = 10;
    format!("{:03}", score)
        .chars()
        .map(|ch| ch.to_digit(radix).expect("format! only gives us digits!"))
        .rev()
        .enumerate()
        .map(|(position, digit)| {
            let x = x - (position as i32) * DIGIT_WIDTH;
            Drawable::sprite(x, y, get_sprite(digit))
        }).collect()
}

/// This is separate from draw_score because in breakout, lives are not
/// padded, but score is, and Rust requires the format string be a literal
pub fn draw_lives(lives: i32, x: i32, y: i32) -> Vec<Drawable> {
    let radix = 10;
    format!("{}", lives)
        .chars()
        .map(|ch| ch.to_digit(radix).expect("format! only gives us digits!"))
        .rev()
        .enumerate()
        .map(|(position, digit)| {
            let x = x - (position as i32) * DIGIT_WIDTH;
            Drawable::sprite(x, y, get_sprite(digit))
        }).collect()
}

/// Parse a number from number_sprites.txt into a SpriteData.
fn load_sprite(data: &[&str]) -> FixedSpriteData {
    // let on_color = Color::white();
    // Should take on_color as an argument, but it needs screen passed in.
    // The current setup doesn't make this easy.
    let on_color = Color::rgb(144, 144, 144);
    let off_color = Color::invisible();
    let mut pixels: Vec<Vec<Color>> = Vec::new();
    for line in data {
        let mut pixel_row = Vec::new();
        for ch in line.chars() {
            if ch == SET {
                pixel_row.push(on_color);
            } else if ch == IGNORE {
                pixel_row.push(off_color);
            } else {
                panic!(
                    "Cannot construct pixel from {}, expected one of (on={}, off={})",
                    ch, SET, IGNORE
                );
            }
        }
        pixels.push(pixel_row);
    }
    let width = pixels[0].len();
    debug_assert!(pixels.iter().all(|row| row.len() == width));
    FixedSpriteData::new(pixels)
}

/// Parse number_sprites.txt, splitting on blank lines.
fn load_digit_sprites() -> Vec<FixedSpriteData> {
    let data = RAW_NUMBER_DATA;

    let mut sprites = Vec::new();
    let mut current = Vec::new();
    for line in data.lines() {
        if line.trim().is_empty() && current.len() > 0 {
            sprites.push(load_sprite(&current));
            current.clear();
        } else {
            current.push(line);
        }
    }
    if current.len() > 0 {
        sprites.push(load_sprite(&current));
    }

    sprites.into_iter().rev().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_we_have_10_fixed_size_digits() {
        for digit in 0..10 {
            let sprite = get_sprite(digit);
            assert_eq!(sprite.width(), DIGIT_WIDTH);
            assert_eq!(sprite.height(), DIGIT_HEIGHT);
            assert_eq!(sprite.find_visible_color().unwrap(), Color::white());
        }
    }
}
