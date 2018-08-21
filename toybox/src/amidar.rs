use super::Input;

pub const GAME_SIZE: (i32, i32) = (240, 240);
pub const AMIDAR_BOARD: &str = include_str!("resources/amidar_default_board");

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn board_included() {
        for line in AMIDAR_BOARD.lines() {
            assert_eq!(Some('='), line.chars().find(|c| *c == '='));
        }
    }
}
