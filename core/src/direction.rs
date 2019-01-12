use super::Input;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}
impl Direction {
    pub fn delta(self) -> (i32, i32) {
        match self {
            Direction::Up => (0, -1),
            Direction::Down => (0, 1),
            Direction::Left => (-1, 0),
            Direction::Right => (1, 0),
        }
    }
    pub fn from_input(buttons: Input) -> Option<Direction> {
        if buttons.up {
            Some(Direction::Up)
        } else if buttons.down {
            Some(Direction::Down)
        } else if buttons.left {
            Some(Direction::Left)
        } else if buttons.right {
            Some(Direction::Right)
        } else {
            None
        }
    }
    pub fn opposite(self) -> Direction {
        match self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }
}
