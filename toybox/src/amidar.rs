use super::Input;
use failure::Error;

// Window constants:
pub mod screen {
    pub const GAME_SIZE: (i32, i32) = (160, 250);
    pub const BOARD_OFFSET: (i32, i32) = (16, 37);
    pub const PLAYER_SIZE: (i32, i32) = (7, 7);
    pub const ENEMY_SIZE: (i32, i32) = (7, 7);
    pub const TILE_SIZE: (i32, i32) = (4, 5);
}

mod world {
    use super::screen;
    pub const SCALE: i32 = 16;
    pub const TILE_SIZE: (i32, i32) = (screen::TILE_SIZE.0 * SCALE, screen::TILE_SIZE.1 * SCALE);
    pub const PLAYER_SIZE: (i32, i32) = (screen::PLAYER_SIZE.0 * SCALE, screen::PLAYER_SIZE.1 * SCALE);
    pub const ENEMY_SIZE: (i32, i32) = (screen::ENEMY_SIZE.0 * SCALE, screen::ENEMY_SIZE.1 * SCALE);
}
pub const AMIDAR_BOARD: &str = include_str!("resources/amidar_default_board");
pub const AMIDAR_ENEMY_POSITIONS_DATA: &str = include_str!("resources/amidar_enemy_positions");

#[derive(Debug, Clone)]
pub struct ScreenPoint {
    pub sx: i32,
    pub sy: i32,
}
impl ScreenPoint {
    fn new(sx: i32, sy: i32) -> ScreenPoint {
        ScreenPoint { sx, sy }
    }
    pub fn pixels(&self) -> (i32, i32) {
        (self.sx, self.sy)
    }
}

/// Strongly-typed vector for "world" positioning in Amidar.
#[derive(Debug, Clone)]
pub struct WorldPoint {
    pub x: i32,
    pub y: i32,
}
impl WorldPoint {
    fn new(x: i32, y: i32) -> WorldPoint {
        WorldPoint { x, y }
    }
    pub fn to_screen(&self) -> ScreenPoint {
        ScreenPoint::new(self.x / world::SCALE, self.y / world::SCALE)
    }
    pub fn to_tile(&self) -> TilePoint {
        let mut tx = self.x / world::TILE_SIZE.0;
        let mut ty = self.y / world::TILE_SIZE.1;
        if self.x < 0 {
            tx -= 1;
        }
        if self.y < 0 {
            ty -= 1;
        }
        TilePoint::new(tx, ty)
    }
    pub fn translate(&self, dx: i32, dy: i32) -> WorldPoint {
        WorldPoint::new(self.x + dx, self.y + dy)
    }
}

/// Strongly-typed vector for "tile" positioning in Amidar.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TilePoint {
    pub tx: i32,
    pub ty: i32,
}
impl TilePoint {
    pub fn new(tx: i32, ty: i32) -> TilePoint {
        TilePoint { tx, ty }
    }
    pub fn to_world(&self) -> WorldPoint {
        WorldPoint::new(self.tx * world::TILE_SIZE.0, self.ty * world::TILE_SIZE.1)
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

#[derive(PartialEq)]
pub enum MovementAI {
    Player,
    EnemyLookupAI { next: u32, path: Vec<u32> },
}

impl MovementAI {
    fn reset(&mut self) {
        match self {
            MovementAI::Player => { },
            MovementAI::EnemyLookupAI { next, path } => {
                *next = 0;
            }
        }
    }
    fn choose_next_tile(&mut self, position: &TilePoint, buttons: &[Input], board: &Board) -> Option<TilePoint> {
        match self {
            MovementAI::Player => {
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

                let target_tile = position.translate(dx, dy);
                // Cannot cross into "empty" space.
                if board.get_tile(&target_tile) != Tile::Empty {
                    Some(target_tile)
                } else {
                    None
                }
            },
            MovementAI::EnemyLookupAI { next, path } => {
                *next = (*next + 1) % (path.len() as u32);
                Some(board.lookup_position(path[*next as usize]))
            }
        }
    }
}

/// Mob is a videogame slang for "mobile" unit. Players and Enemies are the same struct.
pub struct Mob {
    pub ai: MovementAI,
    pub position: WorldPoint,
    speed: i32,
    step: Option<TilePoint>,
}
impl Mob {
    fn new(ai: MovementAI, position: WorldPoint) -> Mob {
        Mob { ai, position, step: None, speed: 8}
    }
    pub fn new_player(position: WorldPoint) -> Mob {
        Mob { ai: MovementAI::Player, position, step: None, speed: 8 }
    }
    fn is_player(&self) -> bool {
        self.ai == MovementAI::Player
    }
    fn reset(&mut self, player_start: &TilePoint, board: &Board) {
        self.step = None;
        self.ai.reset();
        self.position = match self.ai {
            MovementAI::Player => player_start.to_world(),
            MovementAI::EnemyLookupAI { ref path, .. } => board.lookup_position(path[0]).to_world(),
        }
    }
    pub fn update(&mut self, buttons: &[Input], board: &mut Board) {
        // Animate/step player movement.
        let next_target = if let Some(ref target) = self.step {
            // Move player 1 step toward its target:
            let world_target = target.to_world();
            let dx = world_target.x - self.position.x;
            let dy = world_target.y - self.position.y;

            if dx == 0 && dy == 0 {
                // We have reached this target tile:
                if self.is_player() {
                    board.paint(&target);
                }
                None
            } else {
                self.position.x += self.speed * dx.signum();
                self.position.y += self.speed * dy.signum();
                Some(target.clone())
            }
        } else {
            None
        };

        // Rust prevents us from modifying step back to None when we reach it in the above block, since we have bound a reference to the inside of the if-let-Some.
        // We therefore unconditionally return the target from that expression and write it mutably here, where it is obvious it is safe to the rust compiler.
        self.step = next_target;

        // Not an else if -- if a player or enemy reaches a tile they can immediately choose a new target.
        if self.step.is_none() {
            self.step = self.ai.choose_next_tile(&self.position.to_tile(), buttons, board)
        }
    }
}

pub struct Board {
    pub tiles: Vec<Vec<Tile>>,
    pub width: u32,
    pub height: u32,
}

impl Board {
    pub fn try_new() -> Result<Board, Error> {
        let mut tiles = Vec::new();
        for line in AMIDAR_BOARD.lines() {
            // Rust will aggregate errors in collect for us if we give it a type-hint.
            let row: Result<Vec<_>, _> = line.chars().map(|c| Tile::new_from_char(c)).collect();
            // Exit function if row is errorful.
            tiles.push(row?);
        }
        let width = tiles[0].len() as u32;
        let height = tiles.len() as u32;

        Ok(Board { tiles, width, height })
    }
    pub fn paint(&mut self, tile: &TilePoint) {
        self.tiles[tile.ty as usize][tile.tx as usize] = Tile::Painted;
    }
    pub fn make_enemy(&self, positions: Vec<u32>) -> Mob {
        let first = positions[0];
        let ai = MovementAI::EnemyLookupAI { next: 0, path: positions };
        Mob::new(ai, self.lookup_position(first).to_world())
    }
    pub fn lookup_position(&self, position: u32) -> TilePoint {
        let x = position % self.width;
        let y = position / self.width;
        TilePoint::new(x as i32, y as i32)
    }
    fn get_tile(&self, tile: &TilePoint) -> Tile {
        if let Some(row) = self.tiles.get(tile.ty as usize) {
            if let Some(t) = row.get(tile.tx as usize) {
                return *t;
            }
        }
        Tile::Empty
    }
}

pub struct State {
    pub dead: bool,
    pub game_over: bool,
    pub score: i32,
    pub player: Mob,
    pub player_start: TilePoint,
    pub enemies: Vec<Mob>,
    pub board: Board,
}

impl State {
    pub fn new() -> Result<State, Error> {
        let board = Board::try_new()?;

        let mut enemies = Vec::new();
        for enemy_route in AMIDAR_ENEMY_POSITIONS_DATA.lines() {
            let route: Result<Vec<u32>,_> = enemy_route.trim().split(' ').map(|x| x.parse::<u32>()).collect();
            enemies.push(board.make_enemy(route?));
        }
        let player_start = board.lookup_position(511);
        let player = Mob::new_player(player_start.to_world());

        Ok(State {
            dead: false,
            game_over: false,
            score: 0,
            player,
            player_start,
            enemies,
            board: board,
        })
    }
    pub fn reset(&mut self) {
        self.player.reset(&self.player_start, &self.board);
        for enemy in self.enemies.iter_mut() {
            enemy.reset(&self.player_start, &self.board);
        }
    }
    pub fn board_size(&self) -> WorldPoint {
        let th = self.board.height as i32;
        let tw = self.board.width as i32;
        TilePoint::new(tw + 1, th + 1).to_world()
    }
    pub fn update_mut(&mut self, buttons: &[Input]) {
        self.player.update(buttons, &mut self.board);
        for enemy in self.enemies.iter_mut() {
            enemy.update(&[], &mut self.board);

            if self.player.position.to_tile() == enemy.position.to_tile() {
                self.dead = true;
                break;
            }
        }

        if self.dead {
            self.reset();
            self.dead = false;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn board_included() {
        let board_ch: Vec<Vec<char>> = AMIDAR_BOARD
            .lines()
            .map(|line| line.chars().collect::<Vec<char>>())
            .collect();
        for row in board_ch.iter() {
            assert_eq!(Some('='), row.iter().cloned().find(|c| *c == '='));
        }
    }
}
