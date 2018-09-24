use super::graphics::{Color, Drawable, FixedSpriteData};
use super::Direction;
use super::Input;
use super::digit_sprites;
use super::digit_sprites::draw_score;
use failure::Error;
use serde_json;
use std::collections::{HashSet, VecDeque};

// Window constants:
pub mod screen {
    pub const GAME_SIZE: (i32, i32) = (160, 250);
    pub const BOARD_OFFSET: (i32, i32) = (16, 37);
    pub const PLAYER_SIZE: (i32, i32) = (7, 7);
    pub const ENEMY_SIZE: (i32, i32) = (7, 7);
    pub const TILE_SIZE: (i32, i32) = (4, 5);

    pub const LIVES_Y_POS: i32 = 198;
    pub const LIVES_X_POS: i32 = 148; 
    pub const LIVES_X_STEP: i32 = 16;
    
    pub const SCORE_Y_POS: i32 = 198;
    pub const SCORE_X_POS: i32 = LIVES_X_POS - LIVES_X_STEP*3 - 8;
}
pub mod raw_images {
    pub const PLAYER_L1: &[u8] = include_bytes!("resources/amidar/player_l1.png");
    pub const ENEMY_L1: &[u8] = include_bytes!("resources/amidar/enemy_l1.png");
    pub const ENEMY_CHASE_L1: &[u8] = include_bytes!("resources/amidar/enemy_chase_l1.png");
    pub const PAINTED_BOX_BAR: &[u8] = include_bytes!("resources/amidar/painted_box_bar.png");
    pub const BLOCK_TILE_PAINTED_L1: &[u8] = include_bytes!("resources/amidar/block_tile_painted_l1.png");
    pub const BLOCK_TILE_UNPAINTED_L1: &[u8] = include_bytes!("resources/amidar/block_tile_unpainted_l1.png");
}
pub mod images {
    use super::*;
    lazy_static! {
        pub static ref PLAYER_L1: FixedSpriteData = FixedSpriteData::load_png(raw_images::PLAYER_L1);
        pub static ref ENEMY_L1: FixedSpriteData = FixedSpriteData::load_png(raw_images::ENEMY_L1);
        pub static ref ENEMY_JUMP_L1: FixedSpriteData = ENEMY_L1.make_black_version();
        pub static ref ENEMY_CHASE_L1: FixedSpriteData = FixedSpriteData::load_png(raw_images::ENEMY_CHASE_L1);
        pub static ref ENEMY_CAUGHT_L1: FixedSpriteData = ENEMY_CHASE_L1.make_black_version();
        pub static ref PAINTED_BOX_BAR: FixedSpriteData = FixedSpriteData::load_png(raw_images::PAINTED_BOX_BAR);
        pub static ref BLOCK_TILE_PAINTED_L1: FixedSpriteData = FixedSpriteData::load_png(raw_images::BLOCK_TILE_PAINTED_L1);
        pub static ref BLOCK_TILE_UNPAINTED_L1: FixedSpriteData = FixedSpriteData::load_png(raw_images::BLOCK_TILE_UNPAINTED_L1);
    }
}

mod world {
    use super::screen;
    pub const SCALE: i32 = 16;
    pub const TILE_SIZE: (i32, i32) = (screen::TILE_SIZE.0 * SCALE, screen::TILE_SIZE.1 * SCALE);
}
pub const AMIDAR_BOARD: &str = include_str!("resources/amidar_default_board");
pub const AMIDAR_ENEMY_POSITIONS_DATA: &str = include_str!("resources/amidar_enemy_positions");

#[derive(Debug,Clone,Serialize,Deserialize)]
pub struct Config {
    bg_color: Color,
    player_color: Color,
    unpainted_color: Color,
    painted_color: Color,
    enemy_color: Color,
    inner_painted_color: Color,
    start_lives: i32,
    start_jumps: i32,
    render_images: bool,
    chase_time: i32,
    chase_score_bonus: i32,
    jump_time: i32,
    box_bonus: i32,
}

impl Config {
    pub fn colors(&self) -> Vec<&Color> {
        vec![&self.bg_color, &self.enemy_color, &self.inner_painted_color, &self.painted_color, &self.player_color, &self.unpainted_color]
    }
}

impl Default for Config {
    fn default() -> Self {
        Config {
            bg_color: Color::black(),
            player_color: Color::rgb(255, 255, 153),
            unpainted_color: Color::rgb(148, 0, 211),
            painted_color: Color::rgb(255, 255, 30),
            enemy_color: Color::rgb(255, 50, 100),
            inner_painted_color: Color::rgb(255, 255, 0),
            start_lives: 3,
            start_jumps: 4,
            chase_time: 10 * 30, // 10 seconds
            chase_score_bonus: 100,
            jump_time: 2 * 30 + 15, // 2.5 seconds
            render_images: true,
            box_bonus: 50,
        }
    }
}

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
#[derive(Debug, Clone, Serialize, Deserialize)]
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
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
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
    pub fn step(&self, dir: Direction) -> TilePoint {
        let (dx, dy) = dir.delta();
        self.translate(dx, dy)
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GridBox {
    pub top_left: TilePoint,
    pub bottom_right: TilePoint,
    pub painted: bool,
    pub triggers_chase: bool,
}

impl GridBox {
    fn new(top_left: TilePoint, bottom_right: TilePoint, triggers_chase: bool) -> GridBox {
        GridBox {
            top_left,
            bottom_right,
            painted: false,
            triggers_chase
        }
    }
    fn matches(&self, tile: &TilePoint) -> bool {
        let x1 = self.top_left.tx;
        let x2 = self.bottom_right.tx;
        let y1 = self.top_left.ty;
        let y2 = self.bottom_right.ty;

        let xq = tile.tx;
        let yq = tile.ty;

        (x1 <= xq) && (xq <= x2) && (y1 <= yq) && (yq <= y2)
    }
    /// Check whether this box's painting should be updated.
    /// Returns true iff something should change.
    fn should_update_paint(&self, board: &Board) -> bool {
        if self.painted {
            return false;
        }

        let x1 = self.top_left.tx;
        let x2 = self.bottom_right.tx;
        let y1 = self.top_left.ty;
        let y2 = self.bottom_right.ty;

        let top_and_bottom = (x1..(x2+1)).all(|xi| {
            board.is_painted(&TilePoint::new(xi, y1)) && board.is_painted(&TilePoint::new(xi, y2))
        });
        let left_and_right = (y1..(y2+1)).all(|yi| {
            board.is_painted(&TilePoint::new(x1, yi)) && board.is_painted(&TilePoint::new(x2, yi))
        });

        if top_and_bottom && left_and_right {
            return true;
        }
        false
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub enum Tile {
    Empty,
    Unpainted,
    ChaseMarker,
    Painted,
}
impl Tile {
    fn new_from_char(c: char) -> Result<Tile, Error> {
        match c {
            '=' => Ok(Tile::Unpainted),
            'p' => Ok(Tile::Painted),
            'c' => Ok(Tile::ChaseMarker),
            ' ' => Ok(Tile::Empty),
            _ => Err(format_err!("Cannot construct AmidarTile from '{}'", c)),
        }
    }
    fn walkable(self) -> bool {
        match self {
            Tile::Empty => false,
            Tile::ChaseMarker | Tile::Painted | Tile::Unpainted => true,
        }
    }
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub enum MovementAI {
    Player,
    EnemyLookupAI { next: u32, default_route_index: u32 },
}

impl MovementAI {
    fn reset(&mut self) {
        match self {
            &mut MovementAI::Player => {}
            &mut MovementAI::EnemyLookupAI { ref mut next, .. } => {
                *next = 0;
            }
        }
    }
    fn choose_next_tile(
        &mut self,
        position: &TilePoint,
        buttons: Input,
        board: &Board,
    ) -> Option<TilePoint> {
        match self {
            &mut MovementAI::Player => {
                let mut input: Option<Direction> = None;
                if buttons.left {
                    input = Some(Direction::Left);
                } else if buttons.right {
                    input = Some(Direction::Right);
                } else if buttons.up {
                    input = Some(Direction::Up);
                } else if buttons.down {
                    input = Some(Direction::Down);
                }

                input.and_then(|dir| {
                    let target_tile = position.step(dir);
                    if board.get_tile(&target_tile).walkable() {
                        Some(target_tile)
                    } else {
                        None
                    }
                })
            }
            &mut MovementAI::EnemyLookupAI {
                ref mut next,
                default_route_index,
            } => {
                let path = &DEFAULT_ENEMY_ROUTES[default_route_index as usize];
                *next = (*next + 1) % (path.len() as u32);
                Some(board.lookup_position(path[*next as usize]))
            }
        }
    }
}

/// Mob is a videogame slang for "mobile" unit. Players and Enemies are the same struct.
#[derive(Clone, Serialize, Deserialize)]
pub struct Mob {
    pub ai: MovementAI,
    pub position: WorldPoint,
    pub caught: bool,
    speed: i32,
    step: Option<TilePoint>,
    history: VecDeque<u32>,
}
impl Mob {
    fn new(ai: MovementAI, position: WorldPoint) -> Mob {
        Mob {
            ai,
            position,
            step: None,
            caught: false,
            speed: 8,
            history: VecDeque::new(),
        }
    }
    pub fn new_player(position: WorldPoint) -> Mob {
        Mob {
            ai: MovementAI::Player,
            position,
            step: None,
            caught: false,
            speed: 8,
            history: VecDeque::new(),
        }
    }
    fn is_player(&self) -> bool {
        self.ai == MovementAI::Player
    }
    fn reset(&mut self, player_start: &TilePoint, board: &Board) {
        self.step = None;
        self.ai.reset();
        self.position = match self.ai {
            MovementAI::Player => player_start.to_world(),
            MovementAI::EnemyLookupAI {
                default_route_index,
                ..
            } => board
                .lookup_position(DEFAULT_ENEMY_ROUTES[default_route_index as usize][0])
                .to_world(),
        };
        self.history.clear();
    }

    pub fn update(&mut self, buttons: Input, board: &mut Board) -> Option<BoardUpdate> {
        if self.history.is_empty() {
            if let Some(pt) = board.get_junction_id(&self.position.to_tile()) {
                self.history.push_front(pt);
            }
        }

        // Animate/step player movement.
        let next_target = if let Some(ref target) = self.step {
            // Move player 1 step toward its target:
            let world_target = target.to_world();
            let dx = world_target.x - self.position.x;
            let dy = world_target.y - self.position.y;

            if dx == 0 && dy == 0 {
                // We have reached this target tile:
                if let Some(pt) = board.get_junction_id(target) {
                    self.history.push_front(pt);
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
            self.step = self
                .ai
                .choose_next_tile(&self.position.to_tile(), buttons, board)
        }

        // Manage history:
        if self.is_player() {
            board.check_paint(&mut self.history).into_option()
        } else {
            if self.history.len() > 12 {
                let _ = self.history.pop_back();
            }
            None
        }
    }
}

lazy_static! {
    static ref DEFAULT_BOARD: Board = Board::try_new().unwrap();
    static ref DEFAULT_ENEMY_ROUTES: Vec<Vec<u32>> = AMIDAR_ENEMY_POSITIONS_DATA
        .lines()
        .map(|enemy_route| {
            let route: Result<Vec<u32>, _> = enemy_route
                .trim()
                .split(' ')
                .map(|x| x.parse::<u32>())
                .collect();
            route.unwrap()
        }).collect();
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Board {
    pub tiles: Vec<Vec<Tile>>,
    pub width: u32,
    pub height: u32,
    pub junctions: HashSet<u32>,
    pub chase_junctions: HashSet<u32>,
    pub boxes: Vec<GridBox>,
}

#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub struct BoardUpdate {
    pub vertical: i32,
    pub horizontal: i32,
    pub num_boxes: i32,
    pub triggers_chase: bool,
}
impl BoardUpdate {
    fn new() -> BoardUpdate {
        BoardUpdate {
            vertical: 0,
            horizontal: 0,
            num_boxes: 0,
            triggers_chase: false,
        }
    }
    fn happened(&self) -> bool {
        self.vertical != 0 || self.horizontal != 0 || self.num_boxes != 0 || self.triggers_chase
    }
    fn into_option(self) -> Option<Self> {
        if self.happened() {
            Some(self)
        } else {
            None
        }
    }
}

impl Board {
    pub fn fast_new() -> Board {
        DEFAULT_BOARD.clone()
    }
    fn try_new() -> Result<Board, Error> {
        let mut tiles = Vec::new();
        for line in AMIDAR_BOARD.lines() {
            // Rust will aggregate errors in collect for us if we give it a type-hint.
            let row: Result<Vec<_>, _> = line.chars().map(Tile::new_from_char).collect();
            // Exit function if row is errorful.
            tiles.push(row?);
        }
        let width = tiles[0].len() as u32;
        let height = tiles.len() as u32;

        let mut board = Board {
            tiles,
            width,
            height,
            junctions: HashSet::new(),
            chase_junctions: HashSet::new(),
            boxes: Vec::new(),
        };
        board.init_junctions();
        debug_assert!(board.boxes.is_empty());
        board.boxes = board
            .junctions
            .iter()
            .flat_map(|pt| board.junction_corners(*pt))
            .collect();
        Ok(board)
    }

    fn is_corner(&self, tx: i32, ty: i32) -> bool {
        let last_y = (self.height as i32) - 1;
        let last_x = (self.width as i32) - 1;
        (tx == 0 || tx == last_x) && (ty == 0 || ty == last_y)
    }

    fn init_junctions(&mut self) {
        // Only run this function once.
        debug_assert!(self.junctions.is_empty());

        for (y, row) in self.tiles.iter().enumerate() {
            let y = y as i32;
            for (x, cell) in row.iter().enumerate() {
                let x = x as i32;
                if cell.walkable() {
                    let neighbors = [(x + 1, y), (x, y + 1), (x - 1, y), (x, y - 1)];
                    let walkable_neighbors = neighbors
                        .iter()
                        .filter(|&&(nx, ny)| self.get_tile(&TilePoint::new(nx, ny)).walkable())
                        .count();
                    if walkable_neighbors > 2 || self.is_corner(x, y) {
                        let y = y as u32;
                        let x = x as u32;
                        let _ = self.junctions.insert(y * self.width + x);
                        if cell == &Tile::ChaseMarker {
                            self.chase_junctions.insert(y *self.width + x);
                        }
                    }
                }
            }
        }
    }

    fn is_painted(&self, xy: &TilePoint) -> bool {
        self.get_tile(xy) == Tile::Painted
    }

    /// Find the junction in direction ``search`` starting from ``source`` that allows us to walk in ``walkable`` direction.
    fn junction_neighbor(
        &self,
        source: u32,
        search: Direction,
        walkable: Direction,
    ) -> Option<u32> {
        let mut pos = self.lookup_position(source);
        loop {
            pos = pos.step(search);
            let num = self.tile_id(&pos)?;
            if self.junctions.contains(&num) && self.get_tile(&pos.step(walkable)).walkable() {
                return Some(num);
            }
        }
    }

    fn junction_corners(&self, source: u32) -> Option<GridBox> {
        // Find the first junction to the right that lets us go down.
        let right = self.lookup_position(self.junction_neighbor(
            source,
            Direction::Right,
            Direction::Down,
        )?);
        // Find the first junction down that lets us go right.
        let down = self.lookup_position(self.junction_neighbor(
            source,
            Direction::Down,
            Direction::Right,
        )?);
        // There needs to be a bottom_right junction that connects this box.
        let down_right = self.tile_id(&TilePoint::new(right.tx, down.ty))?;

        if self.junctions.contains(&down_right) {
            Some(GridBox::new(
                self.lookup_position(source),
                self.lookup_position(down_right),
                self.chase_junctions.contains(&source)
            ))
        } else {
            None
        }
    }

    fn tile_id(&self, tile: &TilePoint) -> Option<u32> {
        if tile.ty < 0
            || tile.tx < 0
            || tile.ty >= self.height as i32
            || tile.tx >= self.width as i32
        {
            return None;
        }
        let y = tile.ty as u32;
        let x = tile.tx as u32;
        Some(y * self.width + x)
    }

    fn get_junction_id(&self, tile: &TilePoint) -> Option<u32> {
        if let Some(num) = self.tile_id(tile) {
            if self.junctions.contains(&num) {
                Some(num)
            } else {
                None
            }
        } else {
            None
        }
    }

    /// Check whether the painting of segment t1 .. t2 filled any boxes, and return a tuple of
    /// (triggers_chase, count) if so.
    fn check_box_painting(&mut self, t1: &TilePoint, t2: &TilePoint) -> (bool, i32) {
        let indices: Vec<usize> = self
            .boxes
            .iter()
            .enumerate()
            .filter(|&(_, b)| b.matches(t1) || b.matches(t2))
            .filter(|&(_, b)| b.should_update_paint(self))
            .map(|(i, _)| i)
            .collect();

        let updated = indices.len() as i32;
        let mut chase_change = false;
        for i in indices {
            self.boxes[i].painted = true;
            if self.boxes[i].triggers_chase {
                chase_change = true;
            }
        }

        let triggers_chase = chase_change && self.boxes.iter()
            .filter(|b| b.triggers_chase)
            .all(|b| b.painted);

        (triggers_chase, updated)
    }

    fn check_paint(&mut self, player_history: &mut VecDeque<u32>) -> BoardUpdate {
        let mut score_change = BoardUpdate::new();

        if let Some(end) = player_history.front() {
            if let Some(start) = player_history.iter().find(|j| *j != end) {
                // iterate from start..end and paint()

                let t1 = self.lookup_position(*start);
                let t2 = self.lookup_position(*end);
                let dx = (t2.tx - t1.tx).signum();
                let dy = (t2.ty - t1.ty).signum();
                debug_assert!(dx.abs() + dy.abs() == 1);

                let mut newly_painted = false;
                newly_painted |= self.paint(&t1);
                let mut t = t1.clone();
                while t != t2 {
                    t = t.translate(dx, dy);
                    newly_painted |= self.paint(&t);
                }

                // vertical segments give you 1, horizontal give you length
                if newly_painted {
                    if dy.abs() > 0 {
                        score_change.vertical += (t2.ty - t1.ty).abs();
                    } else {
                        score_change.horizontal += (t2.tx - t1.tx).abs();
                    }
                    let (triggers_chase, boxes_painted) = self.check_box_painting(&t1, &t2);
                    score_change.num_boxes += boxes_painted;
                    score_change.triggers_chase = triggers_chase;
                }
            }
        }
        
        if score_change.happened() {
            // Don't forget this location should still be in history:
            let current = *player_history.front().unwrap();
            player_history.clear();
            player_history.push_front(current);
        }

        score_change
    }

    pub fn paint(&mut self, tile: &TilePoint) -> bool {
        let tile = &mut self.tiles[tile.ty as usize][tile.tx as usize];
        if *tile == Tile::Painted {
            false
        } else {
            *tile = Tile::Painted;
            true
        }
    }
    pub fn make_enemy(&self, default_route_index: u32) -> Mob {
        let first = DEFAULT_ENEMY_ROUTES[default_route_index as usize][0];
        let ai = MovementAI::EnemyLookupAI {
            next: 0,
            default_route_index,
        };
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

#[derive(Clone, Serialize, Deserialize)]
pub struct State {
    pub config: Config,
    pub score: i32,
    pub lives: i32,
    pub jumps: i32,
    pub chase_timer: i32,
    pub jump_timer: i32,
    pub player: Mob,
    pub player_start: TilePoint,
    pub enemies: Vec<Mob>,
    pub board: Board,
}

impl State {
    pub fn try_new() -> Result<State, Error> {
        let board = Board::fast_new();

        let mut enemies = Vec::new();
        for (enemy_index, _) in DEFAULT_ENEMY_ROUTES.iter().enumerate() {
            enemies.push(board.make_enemy(enemy_index as u32))
        }
        let player_start = TilePoint::new(31, 15);
        let player = Mob::new_player(player_start.to_world());

        let config = Config::default();

        let mut state = State {
            config: config.clone(),
            lives: config.start_lives,
            score: 0,
            chase_timer: 0,
            jumps: config.start_jumps,
            jump_timer: 0,
            player,
            player_start,
            enemies,
            board,
        };
        state.reset();
        Ok(state)
    }
    pub fn reset(&mut self) {
        self.player.reset(&self.player_start, &self.board);
        self.player
            .history
            .push_front(self.board.get_junction_id(&TilePoint::new(31, 18)).unwrap());
        for enemy in &mut self.enemies {
            enemy.reset(&self.player_start, &self.board);
        }
    }
    pub fn board_size(&self) -> WorldPoint {
        let th = self.board.height as i32;
        let tw = self.board.width as i32;
        TilePoint::new(tw + 1, th + 1).to_world()
    }
    /// Determine whether an enemy and a player are colliding and what to do about it.
    /// returns: (player_dead, enemy_caught)
    fn check_enemy_player_collision(&self, enemy: &Mob, enemy_id: usize) -> EnemyPlayerState {
        if self.player.position.to_tile() == enemy.position.to_tile() {
            if self.chase_timer > 0 {
                if !enemy.caught {
                    EnemyPlayerState::EnemyCatch(enemy_id)
                } else {
                    EnemyPlayerState::Miss
                }
            } else if self.jump_timer > 0 {
                EnemyPlayerState::Miss
            } else {
                EnemyPlayerState::PlayerDeath
            }
        } else {
            // No overlap.
            EnemyPlayerState::Miss
        }
    }
}

#[derive(PartialEq,Eq,Clone,Copy)]
enum EnemyPlayerState {
    Miss,
    PlayerDeath,
    EnemyCatch(usize),
}

pub struct Amidar;
impl super::Simulation for Amidar {
    fn reset_seed(&mut self, seed: u32) {
    }
    fn game_size(&self) -> (i32, i32) {
        screen::GAME_SIZE
    }
    fn new_game(&mut self) -> Box<super::State> {
        Box::new(State::try_new().expect("new_game should succeed."))
    }
    fn new_state_from_json(&self, json_str: &str) -> Result<Box<super::State>, Error> {
        let state: State = serde_json::from_str(json_str)?;
        Ok(Box::new(state))
    }
}

impl super::State for State {
    fn lives(&self) -> i32 {
        self.lives
    }
    fn score(&self) -> i32 {
        self.score
    }
    fn update_mut(&mut self, buttons: Input) {
        let pre_update_score : i32 = self.score();
        if let Some(score_change) = self.player.update(buttons, &mut self.board) {
            self.score += score_change.horizontal;
            // max 1 point for vertical, for some reason.
            self.score += score_change.vertical.signum();
            self.score += self.config.box_bonus * score_change.num_boxes;

            if score_change.triggers_chase {
                self.chase_timer = self.config.chase_time;
            }
        }

        if self.chase_timer > 0 {
            self.chase_timer -= 1;
        } else if self.jump_timer > 0 { // only support jump when not chasing.
            self.jump_timer -= 1;
        } else if (buttons.button1 || buttons.button2) && self.jumps > 0 {
            self.jump_timer = self.config.jump_time;
            self.jumps-=1;
        }

        let mut dead = false;
        let mut changes: Vec<EnemyPlayerState> = Vec::new();
        
        // check-collisions after player move:
        for (i,e) in self.enemies.iter().enumerate() {
            let state = self.check_enemy_player_collision(e, i);
            if state != EnemyPlayerState::Miss {
                changes.push(state);
            } 
        }

        // move enemies:
        for e in self.enemies.iter_mut() {
            e.update(Input::default(), &mut self.board);
        }
        
        // check-collisions again (so we can't run through enemies):
        for (i,e) in self.enemies.iter().enumerate() {
            let state = self.check_enemy_player_collision(e, i);
            if state != EnemyPlayerState::Miss {
                changes.push(state);
            } 
        }
        
        // Process EnemyPlayerState that were interesting!
        for change in changes {
            match change {
                EnemyPlayerState::Miss => {
                    // This was filtered out.
                },
                EnemyPlayerState::PlayerDeath => {
                    dead = true;
                    println!("ded");
                    break;
                },
                EnemyPlayerState::EnemyCatch(eid) => {
                    if !self.enemies[eid].caught {
                        self.score += self.config.chase_score_bonus;
                        self.enemies[eid].caught = true;
                    }
                },
            }
        }

        if dead {
            self.jumps = self.config.start_jumps;
            self.lives -= 1;
            self.score = pre_update_score;
            self.reset();
        }
    }

    fn draw(&self) -> Vec<Drawable> {
        let mut output = Vec::new();
        output.push(Drawable::rect(
            self.config.bg_color,
            0,
            0,
            screen::GAME_SIZE.0,
            screen::GAME_SIZE.1,
        ));
        if self.lives <= 0 {
            return output;
        }

        let (tile_w, tile_h) = screen::TILE_SIZE;
        let (offset_x, offset_y) = screen::BOARD_OFFSET;

        for (ty, row) in self.board.tiles.iter().enumerate() {
            let ty = ty as i32;
            for (tx, tile) in row.iter().enumerate() {
                let tx = tx as i32;

                if self.config.render_images {
                    let tile_sprite: &FixedSpriteData = match tile {
                        &Tile::Painted => &images::BLOCK_TILE_PAINTED_L1,
                        &Tile::Unpainted | &Tile::ChaseMarker => &images::BLOCK_TILE_UNPAINTED_L1,
                        &Tile::Empty => continue,
                    };
                    output.push(Drawable::sprite(
                        offset_x + tx * tile_w,
                        offset_y + ty * tile_h,
                        tile_sprite.clone(),
                    ));
                } else {
                    let tile_color = match tile {
                        &Tile::Painted => self.config.painted_color,
                        &Tile::Unpainted | &Tile::ChaseMarker => self.config.unpainted_color,
                        &Tile::Empty => continue,
                    };
                    output.push(Drawable::rect(
                        tile_color,
                        offset_x + tx * tile_w,
                        offset_y + ty * tile_h,
                        tile_w,
                        tile_h,
                    ));
                }
            }
        }

        for inner_box in self.board.boxes.iter().filter(|b| b.painted) {
            if self.config.render_images {
                let top_left_in = inner_box.top_left.translate(1,1);
                let x1 = top_left_in.tx;
                let x2 = inner_box.bottom_right.tx;
                let y1 = top_left_in.ty;
                let y2 = inner_box.bottom_right.ty;

                // generate all boxes inside:
                for x in x1..x2 {
                    for y in y1..y2 {
                        let pt = TilePoint::new(x,y).to_world().to_screen();
                        output.push(Drawable::sprite(
                            pt.sx + offset_x, pt.sy + offset_y, images::PAINTED_BOX_BAR.clone()));
                    }
                }

            } else {
                let origin = inner_box.top_left.translate(1, 1).to_world().to_screen();
                let dest = inner_box.bottom_right.to_world().to_screen();
                let w = dest.sx - origin.sx;
                let h = dest.sy - origin.sy;
                output.push(Drawable::rect(
                    self.config.inner_painted_color,
                    offset_x + origin.sx,
                    offset_y + origin.sy,
                    w,
                    h,
                ));
            }
        }

        let (player_x, player_y) = self.player.position.to_screen().pixels();
        let (player_w, player_h) = screen::PLAYER_SIZE;
        if self.config.render_images {
            output.push(Drawable::sprite(
                offset_x + player_x - 1, 
                offset_y + player_y - 1,
                images::PLAYER_L1.clone()
            ))
        } else {
            output.push(Drawable::rect(
                self.config.player_color,
                offset_x + player_x - 1,
                offset_y + player_y - 1,
                player_w,
                player_h,
            ));
        }

        for enemy in &self.enemies {
            let (x, y) = enemy.position.to_screen().pixels();
            let (w, h) = screen::ENEMY_SIZE;

            if self.config.render_images {
                output.push(Drawable::sprite(
                    offset_x + x - 1, 
                    offset_y + y - 1,
                    if self.chase_timer > 0 { 
                        if enemy.caught {
                            images::ENEMY_JUMP_L1.clone()
                        } else {
                            images::ENEMY_CHASE_L1.clone() 
                        }
                    } else if self.jump_timer > 0 {
                        images::ENEMY_JUMP_L1.clone()
                    } else { 
                        images::ENEMY_L1.clone() 
                    }
                ))
            } else {
                output.push(Drawable::rect(
                    self.config.enemy_color,
                    offset_x + x - 1,
                    offset_y + y - 1,
                    w,
                    h,
                ));
            }
        }

        output.extend(draw_score(self.score, screen::SCORE_X_POS, screen::SCORE_Y_POS));
        for i in 0..self.lives {
            output.push(Drawable::rect(self.config.player_color,
                screen::LIVES_X_POS - i * screen::LIVES_X_STEP,
                screen::LIVES_Y_POS,
                1,
                digit_sprites::DIGIT_HEIGHT+1
            ))
        }

        output
    }
    
    fn to_json(&self) -> String {
        serde_json::to_string(self).expect("Should be no JSON Serialization Errors.")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_colors_unique_in_gray() {
        let config = Config::default();
        let num_colors = config.colors().len();
        let uniq_grays: HashSet<u8> = config.colors().into_iter().map(|c| c.grayscale_byte()).collect();
        // Don't allow a grayscale agent to be confused where a human wouldn't be.
        assert_eq!(uniq_grays.len(), num_colors);
    }

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

    #[test]
    fn board_corners() {
        let board = Board::fast_new();
        assert!(board.is_corner(0, 0));
        assert!(board.is_corner(0, 30));
        assert!(board.is_corner(31, 0));
        assert!(board.is_corner(31, 30));
    }
    #[test]
    fn player_start_position() {
        let board = Board::fast_new();
        assert_eq!(TilePoint::new(31, 15), board.lookup_position(511));
        assert!(board.get_junction_id(&TilePoint::new(31, 18)).is_some());
    }

    #[test]
    fn num_grid_boxes() {
        let board = Board::fast_new();
        let mut ordered = board.boxes.clone();
        ordered.sort_by_key(|it| it.top_left.tx + it.top_left.ty * 32);
        for gb in ordered {
            println!("Box-found: {:?}", gb.top_left);
        }
        assert_eq!(board.boxes.len(), 29);
    }

    #[test]
    fn test_load_png() {
        let img = &images::PLAYER_L1;
    }
}
