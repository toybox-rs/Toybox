use super::Input;
use failure::Error;

pub const GAME_SIZE: (i32, i32) = (160, 250);
pub const BOARD_OFFSET: (i32, i32) = (16, 37);
pub const TILE_SIZE: (i32, i32) = (4, 5);
pub const ENTITY_SIZE: (i32, i32) = (7, 7);
pub const AMIDAR_BOARD: &str = include_str!("resources/amidar_default_board");

/// Strongly-typed vector for "world" positioning in Amidar.
#[derive(Debug, Clone)]
pub struct WorldPoint {
    pub x: i32,
    pub y: i32,
}
impl WorldPoint {
    pub fn new(x: i32, y: i32) -> WorldPoint {
        WorldPoint { x, y }
    }
    pub fn to_tile(&self) -> TilePoint {
        let mut tx = self.x / TILE_SIZE.0;
        let mut ty = self.y / TILE_SIZE.1;
        if self.x < 0 {
            tx -= 1;
        }
        if self.y < 0 {
            ty -= 1;
        }
        TilePoint::new(tx, ty)
    }
    pub fn pixels(&self) -> (i32, i32) {
        (self.x, self.y)
    }
    pub fn translate(&self, dx: i32, dy: i32) -> WorldPoint {
        WorldPoint::new(self.x + dx, self.y + dy)
    }
}

/// Strongly-typed vector for "tile" positioning in Amidar.
#[derive(Debug, Clone)]
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
    pub fn translate(&self, dx: i32, dy: i32) -> TilePoint {
        TilePoint::new(self.tx + dx, self.ty + dy)
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
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
            _ => Err(format_err!("Cannot construct AmidarTile from '{}'", c)),
        }
    }
}

pub fn get_board_chars() -> Vec<Vec<char>> {
    AMIDAR_BOARD
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect()
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
    player_target: Option<TilePoint>,
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
            player: TilePoint::new(4, 0).to_world(),
            player_target: None,
            enemies: Vec::new(),
            board: board_tiles,
        })
    }
    pub fn board_size(&self) -> WorldPoint {
        let th = self.board.len() as i32;
        let tw = self.board[0].len() as i32;
        TilePoint::new(tw + 1, th + 1).to_world()
    }
    fn get_tile(&self, tile: &TilePoint) -> Tile {
        if let Some(row) = self.board.get(tile.ty as usize) {
            if let Some(t) = row.get(tile.tx as usize) {
                return *t;
            }
        }
        Tile::Empty
    }
    fn paint(&mut self, tile: &TilePoint) {}
    pub fn update_mut(&mut self, buttons: &[Input]) {
        // Animate/step player movement.
        let next_target = if let Some(ref target) = self.player_target {
            // Move player 1 step toward its target:
            let world_target = target.to_world();
            let dx = world_target.x - self.player.x;
            let dy = world_target.y - self.player.y;

            if dx == 0 && dy == 0 {
                // We have reached this target tile:
                self.board[target.ty as usize][target.tx as usize] = Tile::Painted;
                None
            } else {
                self.player.x += dx.signum();
                self.player.y += dy.signum();
                Some(target.clone())
            }
        } else {
            None
        };
        // Rust prevents us from modifying target back to None when we reach it in the above block, since we have bound a reference to the inside of the Some.
        // We therefore unconditionally return the target and write it mutably here.
        self.player_target = next_target;

        // Not an else if -- if a player reaches a tile they can immediately choose a new target.
        if self.player_target.is_none() {
            let left = buttons.contains(&Input::Left);
            let right = buttons.contains(&Input::Right);
            let up = buttons.contains(&Input::Up);
            let down = buttons.contains(&Input::Down);

            let mut dx = 0;
            let mut dy = 0;
            if left {
                dx = -1;
            } else if right {
                dx = 1;
            } else if up {
                dy = -1;
            } else if down {
                dy = 1;
            }

            let target_tile = self.player.to_tile().translate(dx, dy);
            // Cannot cross into "empty" space.
            if self.get_tile(&target_tile) != Tile::Empty {
                self.player_target = Some(target_tile);
            }
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
            assert_eq!(Some('='), row.iter().cloned().find(|c| *c == '='));
        }
    }
}
