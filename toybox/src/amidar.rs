use failure::Error;
use super::Input;

pub const GAME_SIZE: (i32, i32) = (160, 250);
pub const BOARD_OFFSET: (i32, i32) = (16,37);
pub const TILE_SIZE: (i32, i32) = (4,5);
pub const ENTITY_SIZE: (i32, i32) = (7,7);
pub const AMIDAR_BOARD: &str = include_str!("resources/amidar_default_board");

/// Strongly-typed vector for "world" positioning in Amidar.
pub struct WorldPoint {
    pub x: i32,
    pub y: i32,
}
impl WorldPoint {
    pub fn new(x: i32, y: i32) -> WorldPoint {
        WorldPoint { x, y }
    }
    pub fn to_tile(&self) -> TilePoint {
        TilePoint::new(self.x/TILE_SIZE.0, self.y/TILE_SIZE.1)
    }
    pub fn pixels(&self) -> (i32, i32) {
        (self.x, self.y)
    }
}

// Strongly-typed vector for "tile" positioning in Amidar.
pub struct TilePoint {
    pub tx: i32,
    pub ty: i32,
}
impl TilePoint {
    pub fn new(tx: i32, ty: i32) -> TilePoint {
        TilePoint { tx, ty }
    }
    pub fn to_world(&self) -> WorldPoint {
        WorldPoint::new(self.tx * TILE_SIZE.0, self.ty * TILE_SIZE.1)
    }
}

pub enum Tile {
    Empty,
    Unpainted,
    Painted,
}
impl Tile {
    fn new_from_char(c: char) -> Result<Tile, Error> {
        match c {
            '=' => Ok(Tile::Unpainted),
            ' ' => Ok(Tile::Empty),
            _ => Err(format_err!("Cannot construct AmidarTile from '{}'", c))
        }
    }
}


pub fn get_board_chars() -> Vec<Vec<char>> {
    AMIDAR_BOARD.lines().map(|line| line.chars().collect::<Vec<char>>()).collect()
}

pub enum MovementAI {
    Player,
    OutsideEnemy,
    DiagonalEnemy,
}

pub struct Enemy {
    pub ai: MovementAI,
    pub position: WorldPoint,
}
impl Enemy {
    fn new(ai: MovementAI, position: WorldPoint) -> Enemy {
        Enemy { ai, position }
    }
}

pub struct State {
    pub game_over: bool,
    pub score: i32,
    pub player: WorldPoint,
    pub enemies: Vec<Enemy>,
    pub board: Vec<Vec<Tile>>,
}

impl State {
    pub fn new() -> Result<State, Error> {
        let board_data = get_board_chars();
        let mut board_tiles = Vec::new();
        for line in AMIDAR_BOARD.lines() {
            // Rust will aggregate errors in collect for us if we give it a type-hint.
            let row: Result<Vec<_>, _> = line.chars().map(|c| Tile::new_from_char(c)).collect();
            // Exit function if row is errorful.
            board_tiles.push(row?);
        }
        
        Ok(State { 
            game_over: false,
            score: 0,
            player: TilePoint::new(1,1).to_world(),
            enemies: Vec::new(),
            board: board_tiles,
        })
    }
    pub fn board_size(&self) -> WorldPoint {
        let th = self.board.len() as i32;
        let tw = self.board[0].len() as i32;
        TilePoint::new(tw+1, th+1).to_world()
    }
    pub fn update_mut(&mut self, buttons: &[Input]) {
        let left = buttons.contains(&Input::Left);
        let right = buttons.contains(&Input::Right);
        let up = buttons.contains(&Input::Up);
        let down = buttons.contains(&Input::Down);

        if left {
            self.player.x -= 1;
        } else if right {
            self.player.x += 1;
        } else if up {
            self.player.y -= 1;
        } else if down {
            self.player.y += 1;
        }

    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn board_included() {
        let board_ch = get_board_chars();
        for row in board_ch.iter() {
            assert_eq!(Some('='), row.iter().find(|c| *c == '='));
        }
    }
}
