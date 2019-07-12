use super::digit_sprites::{draw_score, DIGIT_HEIGHT};
use serde_json;
use std::collections::{HashSet, VecDeque};
use toybox_core;
use toybox_core::graphics::{Color, Drawable, FixedSpriteData};
use toybox_core::random;
use toybox_core::{AleAction, Direction, Input, QueryError};
use types::*;

use rand::seq::SliceRandom;

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
    pub const SCORE_X_POS: i32 = LIVES_X_POS - LIVES_X_STEP * 3 - 8;
}
pub mod raw_images {
    pub const PLAYER_L1: &[u8] = include_bytes!("resources/amidar/player_l1.png");
    pub const PLAYER_L2: &[u8] = include_bytes!("resources/amidar/player_l2.png");
    pub const ENEMY_L1: &[u8] = include_bytes!("resources/amidar/enemy_l1.png");
    pub const ENEMY_L2: &[u8] = include_bytes!("resources/amidar/enemy_l2.png");
    pub const ENEMY_CHASE_L1: &[u8] = include_bytes!("resources/amidar/enemy_chase_l1.png");
    pub const ENEMY_CHASE_L2: &[u8] = include_bytes!("resources/amidar/enemy_chase_l1.png");
    pub const PAINTED_BOX_BAR: &[u8] = include_bytes!("resources/amidar/painted_box_bar.png");

    pub const BLOCK_TILE_PAINTED_L1: &[u8] =
        include_bytes!("resources/amidar/block_tile_painted_l1.png");
    pub const BLOCK_TILE_PAINTED_L2: &[u8] =
        include_bytes!("resources/amidar/block_tile_painted_l2.png");
    pub const BLOCK_TILE_UNPAINTED_L1: &[u8] =
        include_bytes!("resources/amidar/block_tile_unpainted_l1.png");
    pub const BLOCK_TILE_UNPAINTED_L2: &[u8] =
        include_bytes!("resources/amidar/block_tile_unpainted_l2.png");
}
pub mod images {
    use super::*;
    lazy_static! {
        // Level 1 images
        pub static ref PLAYER_L1: FixedSpriteData =
            FixedSpriteData::load_png(raw_images::PLAYER_L1);
        pub static ref ENEMY_L1: FixedSpriteData = FixedSpriteData::load_png(raw_images::ENEMY_L1);
        pub static ref ENEMY_JUMP_L1: FixedSpriteData = ENEMY_L1.make_black_version();
        pub static ref ENEMY_CHASE_L1: FixedSpriteData =
            FixedSpriteData::load_png(raw_images::ENEMY_CHASE_L1);
        pub static ref ENEMY_CAUGHT_L1: FixedSpriteData = ENEMY_CHASE_L1.make_black_version();
        pub static ref BLOCK_TILE_PAINTED_L1: FixedSpriteData =
            FixedSpriteData::load_png(raw_images::BLOCK_TILE_PAINTED_L1);
        pub static ref BLOCK_TILE_UNPAINTED_L1: FixedSpriteData =
            FixedSpriteData::load_png(raw_images::BLOCK_TILE_UNPAINTED_L1);


        // Level 2 images
        pub static ref PLAYER_L2: FixedSpriteData =
            FixedSpriteData::load_png(raw_images::PLAYER_L2);
        pub static ref ENEMY_L2: FixedSpriteData = FixedSpriteData::load_png(raw_images::ENEMY_L2);
        pub static ref ENEMY_JUMP_L2: FixedSpriteData = ENEMY_L2.make_black_version();
        pub static ref ENEMY_CHASE_L2: FixedSpriteData =
            FixedSpriteData::load_png(raw_images::ENEMY_CHASE_L2);
        pub static ref ENEMY_CAUGHT_L2: FixedSpriteData = ENEMY_CHASE_L2.make_black_version();
        pub static ref BLOCK_TILE_PAINTED_L2: FixedSpriteData =
            FixedSpriteData::load_png(raw_images::BLOCK_TILE_PAINTED_L2);
        pub static ref BLOCK_TILE_UNPAINTED_L2: FixedSpriteData =
            FixedSpriteData::load_png(raw_images::BLOCK_TILE_UNPAINTED_L2);


        pub static ref PAINTED_BOX_BAR: FixedSpriteData =
            FixedSpriteData::load_png(raw_images::PAINTED_BOX_BAR);

    }
}

mod world {
    use super::screen;
    pub const SCALE: i32 = 16;
    pub const TILE_SIZE: (i32, i32) = (screen::TILE_SIZE.0 * SCALE, screen::TILE_SIZE.1 * SCALE);
}
pub const AMIDAR_BOARD: &str = include_str!("resources/amidar_default_board");
pub const AMIDAR_ENEMY_POSITIONS_DATA: &str = include_str!("resources/amidar_enemy_positions");

mod inits {
    pub const ENEMY_STARTING_SPEED: i32 = 10;
    pub const PLAYER_SPEED: i32 = 8;
}

impl Amidar {
    pub fn colors(&self) -> Vec<&Color> {
        vec![
            &self.bg_color,
            &self.enemy_color,
            &self.inner_painted_color,
            &self.painted_color,
            &self.player_color,
            &self.unpainted_color,
        ]
    }
}

impl Default for Amidar {
    fn default() -> Self {
        Amidar {
            rand: random::Gen::new_from_seed(13),
            board: AMIDAR_BOARD.lines().map(|s| s.to_owned()).collect(),
            player_start: TilePoint::new(31, 15),
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
            default_board_bugs: true,
            // limit number of junctions remembered to something greater than two.
            history_limit: 12,
            enemies: (0..DEFAULT_ENEMY_ROUTES.len())
                .map(|idx| MovementAI::EnemyLookupAI {
                    next: 0,
                    default_route_index: idx as u32,
                })
                .collect(),
            level: 1,
            enemy_starting_speed: inits::ENEMY_STARTING_SPEED,
            player_speed: inits::PLAYER_SPEED,
        }
    }
}

impl ScreenPoint {
    fn new(sx: i32, sy: i32) -> ScreenPoint {
        ScreenPoint { sx, sy }
    }
    pub fn pixels(&self) -> (i32, i32) {
        (self.sx, self.sy)
    }
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

impl TilePoint {
    pub fn new(tx: i32, ty: i32) -> TilePoint {
        TilePoint { tx, ty }
    }
    pub fn manhattan_dist(&self, other: &TilePoint) -> i32 {
        (self.tx - other.tx).abs() + (self.ty - other.ty).abs()
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

impl GridBox {
    fn new(top_left: TilePoint, bottom_right: TilePoint, triggers_chase: bool) -> GridBox {
        GridBox {
            top_left,
            bottom_right,
            painted: false,
            triggers_chase,
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

        let top_and_bottom = (x1..(x2 + 1)).all(|xi| {
            board.is_painted(&TilePoint::new(xi, y1)) && board.is_painted(&TilePoint::new(xi, y2))
        });
        let left_and_right = (y1..(y2 + 1)).all(|yi| {
            board.is_painted(&TilePoint::new(x1, yi)) && board.is_painted(&TilePoint::new(x2, yi))
        });

        if top_and_bottom && left_and_right {
            return true;
        }
        false
    }
}

impl Tile {
    fn new_from_char(c: char) -> Result<Tile, String> {
        match c {
            '=' => Ok(Tile::Unpainted),
            'p' => Ok(Tile::Painted),
            'c' => Ok(Tile::ChaseMarker),
            ' ' => Ok(Tile::Empty),
            _ => Err(format!("Cannot construct AmidarTile from '{}'", c)),
        }
    }
    pub fn walkable(self) -> bool {
        match self {
            Tile::Empty => false,
            Tile::ChaseMarker | Tile::Painted | Tile::Unpainted => true,
        }
    }
    pub fn needs_paint(self) -> bool {
        match self {
            Tile::Painted | Tile::Empty => false,
            Tile::ChaseMarker | Tile::Unpainted => true,
        }
    }
}

impl MovementAI {
    /// Resetting the mob AI state after player death.
    fn reset(&mut self) {
        match self {
            &mut MovementAI::Player => {}
            &mut MovementAI::EnemyLookupAI { ref mut next, .. } => {
                *next = 0;
            }
            &mut MovementAI::EnemyPerimeterAI { .. } => {}
            &mut MovementAI::EnemyAmidarMvmt {
                ref mut vert,
                start_vert,
                ref mut horiz,
                start_horiz,
                ..
            } => {
                *vert = start_vert;
                *horiz = start_horiz;
            }
            &mut MovementAI::EnemyRandomMvmt {
                ref mut dir,
                start_dir,
                ..
            } => {
                *dir = start_dir;
            }
            &mut MovementAI::EnemyTargetPlayer {
                start_dir,
                ref mut dir,
                ref mut player_seen,
                ..
            } => {
                *dir = start_dir;
                *player_seen = None;
            }
        }
    }
    fn choose_next_tile(
        &mut self,
        position: &TilePoint,
        buttons: Input,
        board: &Board,
        player: Option<Mob>,
        rng: &mut random::Gen,
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
            &mut MovementAI::EnemyPerimeterAI { .. } => {
                let perimeter = board.get_perimeter(position);
                let mut tilepoint = None;
                for dir in perimeter {
                    let go = match dir {
                        Direction::Up => Direction::Right,
                        Direction::Down => Direction::Left,
                        Direction::Right => Direction::Down,
                        Direction::Left => Direction::Up,
                    };
                    let tp = board.can_move(position, go);
                    tilepoint = tilepoint.or(tp)
                }
                tilepoint
            }
            &mut MovementAI::EnemyAmidarMvmt {
                ref mut vert,
                ref mut horiz,
                ..
            } => {
                let maybe_vert: Option<TilePoint> = board.can_move(position, *vert);
                let perimeter: Vec<Direction> = board.get_perimeter(position);
                let maybe_horiz = board.can_move(position, *horiz);
                if perimeter.contains(vert) {
                    *vert = vert.opposite();
                }
                if maybe_vert.is_some() {
                    // Check to see if we are on the left or right sides
                    if perimeter.contains(&Direction::Left) || perimeter.contains(&Direction::Right)
                    {
                        // Then try to move horizontally first.
                        if maybe_horiz.is_some() {
                            return maybe_horiz;
                        }
                    }
                    maybe_vert
                } else if maybe_horiz.is_some() {
                    maybe_horiz
                } else {
                    // Flip horiz
                    *horiz = horiz.opposite();
                    board.can_move(position, *horiz)
                }
            }
            &mut MovementAI::EnemyRandomMvmt { ref mut dir, .. } => {
                let directions = &[
                    Direction::Up,
                    Direction::Down,
                    Direction::Left,
                    Direction::Right,
                ];
                let eligible: Vec<(&Direction, Option<TilePoint>)> = directions
                    .iter()
                    .map(|d| (d, board.can_move(position, *d)))
                    .filter(|(_, tp)| tp.is_some())
                    .collect();
                let (d, tp) = eligible.choose(rng).cloned().unwrap();
                let tp_default = board.can_move(position, *dir);
                if board.is_junction(position) || tp_default.is_none() {
                    // Move to the randomly selected tile point, in its dir.
                    *dir = *d;
                    return tp;
                }
                tp_default
            }
            &mut MovementAI::EnemyTargetPlayer {
                ref mut player_seen,
                ref mut dir,
                vision_distance,
                ..
            } => {
                let player = player.unwrap();
                assert!(player.is_player());
                let player_tile = player.position.to_tile();
                let px = player_tile.tx;
                let py = player_tile.ty;
                if board.is_line_of_sight(position, &player_tile)
                    && position.manhattan_dist(&player_tile) <= vision_distance
                {
                    // The player is currently within view
                    *player_seen = Some(player_tile);
                    *dir = if px == position.tx {
                        if py < position.ty {
                            Direction::Up
                        } else {
                            Direction::Down
                        }
                    } else {
                        if px < position.tx {
                            Direction::Left
                        } else {
                            Direction::Right
                        }
                    };
                    board.can_move(position, *dir)
                } else {
                    if player_seen.is_some() && *position == player_seen.clone().unwrap() {
                        // If we've caught up with the player's last known position,
                        // the trail is stale. Reset.
                        *player_seen = None;
                    }
                    if player_seen.is_some() {
                        // We are still tracking the player
                        board.can_move(position, *dir)
                    } else {
                        // Explore
                        let tp_default = board.can_move(position, *dir);
                        if board.is_junction(position) || tp_default.is_none() {
                            let directions = &[
                                Direction::Up,
                                Direction::Down,
                                Direction::Left,
                                Direction::Right,
                            ];
                            let eligible: Vec<(&Direction, Option<TilePoint>)> = directions
                                .iter()
                                .map(|d| (d, board.can_move(position, *d)))
                                .filter(|(_, tp)| tp.is_some())
                                .collect();
                            let (d, tp) = eligible.choose(rng).cloned().unwrap();
                            *dir = *d;
                            tp
                        } else {
                            tp_default
                        }
                    }
                }
            }
        }
    }
}

impl Mob {
    fn new(ai: MovementAI, position: WorldPoint, speed: i32) -> Mob {
        Mob {
            ai,
            position,
            step: None,
            caught: false,
            speed: speed,
            history: VecDeque::new(),
        }
    }
    pub fn new_player(position: WorldPoint, speed: i32) -> Mob {
        Mob {
            ai: MovementAI::Player,
            position,
            step: None,
            caught: false,
            speed: speed,
            history: VecDeque::new(),
        }
    }
    fn is_player(&self) -> bool {
        self.ai == MovementAI::Player
    }
    fn change_speed(&mut self, new_speed: i32) {
        self.speed = new_speed;
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
            MovementAI::EnemyPerimeterAI { .. } => TilePoint::new(0, 0).to_world(),
            MovementAI::EnemyAmidarMvmt { ref start, .. } => start.clone().to_world(),
            MovementAI::EnemyRandomMvmt { ref start, .. } => start.clone().to_world(),
            MovementAI::EnemyTargetPlayer { ref start, .. } => start.clone().to_world(),
        };
        self.history.clear();
    }

    pub fn update(
        &mut self,
        buttons: Input,
        board: &mut Board,
        player: Option<Mob>,
        history_limit: u32,
        rng: &mut random::Gen,
    ) -> Option<BoardUpdate> {
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
                if dx.abs() < self.speed && dy.abs() < self.speed {
                    self.position.x += dx;
                    self.position.y += dy;
                    if let Some(pt) = board.get_junction_id(target) {
                        self.history.push_front(pt);
                    }
                    None
                } else {
                    self.position.x += self.speed * dx.signum();
                    self.position.y += self.speed * dy.signum();
                    Some(target.clone())
                }
            }
        } else {
            None
        };

        // Rust prevents us from modifying step back to None when we reach it in the above block, since we have bound a reference to the inside of the if-let-Some.
        // We therefore unconditionally return the target from that expression and write it mutably here, where it is obvious it is safe to the rust compiler.
        self.step = next_target;

        // Not an else if -- if a player or enemy reaches a tile they can immediately choose a new target.
        if self.step.is_none() {
            self.step =
                self.ai
                    .choose_next_tile(&self.position.to_tile(), buttons, board, player, rng)
        }

        // Manage history:
        if self.is_player() {
            board.check_paint(&mut self.history).into_option()
        } else {
            // Each moving object in Amidar keeps track of which junctions it has visited. Here, we
            // make sure that datastructure does not grow unbounded with time; limiting it to
            // what is defined in the config.

            if self.history.len() > (history_limit as usize) {
                let _ = self.history.pop_back();
            }
            None
        }
    }
}

lazy_static! {
    static ref DEFAULT_BOARD: Board = Board::try_new(
        &AMIDAR_BOARD
            .lines()
            .map(|s| s.to_owned())
            .collect::<Vec<_>>()
    )
    .unwrap();
    static ref DEFAULT_ENEMY_ROUTES: Vec<Vec<u32>> = AMIDAR_ENEMY_POSITIONS_DATA
        .lines()
        .map(|enemy_route| {
            let route: Result<Vec<u32>, _> = enemy_route
                .trim()
                .split(' ')
                .map(|x| x.parse::<u32>())
                .collect();
            route.unwrap()
        })
        .collect();
}

impl BoardUpdate {
    fn new() -> BoardUpdate {
        BoardUpdate {
            junctions: None,
            vertical: 0,
            horizontal: 0,
            num_boxes: 0,
            triggers_chase: false,
        }
    }
    fn happened(&self) -> bool {
        self.junctions.is_some()
            || self.vertical != 0
            || self.horizontal != 0
            || self.num_boxes != 0
            || self.triggers_chase
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
    fn try_new(lines: &[String]) -> Result<Board, String> {
        let mut tiles = Vec::new();
        for line in lines {
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

    fn is_line_of_sight(&self, p1: &TilePoint, p2: &TilePoint) -> bool {
        if p1 == p2 {
            // I hope this does structural equality.
            true
        } else if p1.ty == p2.ty {
            // get the min X and check to see if every tile moving right between
            // the min and the target is track.
            let (leftest, rightest) = if p1.tx < p2.tx { (p1, p2) } else { (p2, p1) };
            let mut x = leftest.tx;
            let y = p1.ty;
            while x < rightest.tx {
                if self
                    .can_move(&TilePoint::new(x, y), Direction::Right)
                    .is_some()
                {
                    x += 1;
                } else {
                    return false;
                }
            }
            true
        } else if p1.tx == p2.tx {
            // get the min Y and check to see if every time moving down between
            // the min and the target is track.
            let (toppest, downest) = if p1.ty < p2.ty { (p1, p2) } else { (p2, p1) };
            let x = p1.tx;
            let mut y = toppest.ty;
            while y < downest.ty {
                if self
                    .can_move(&TilePoint::new(x, y), Direction::Down)
                    .is_some()
                {
                    y += 1;
                } else {
                    return false;
                }
            }
            true
        } else {
            false
        }
    }

    fn get_perimeter(&self, position: &TilePoint) -> Vec<Direction> {
        let tx = position.tx;
        let ty = position.ty;
        let mut retval = Vec::new();

        let last_y = (self.height as i32) - 1;
        let last_x = (self.width as i32) - 1;

        if ty == 0 {
            retval.push(Direction::Up)
        } else if ty == last_y {
            retval.push(Direction::Down)
        }

        if tx == 0 {
            retval.push(Direction::Left)
        } else if tx == last_x {
            retval.push(Direction::Right)
        }
        assert!(retval.len() < 3);
        retval
    }

    fn can_move(&self, position: &TilePoint, dir: Direction) -> Option<TilePoint> {
        let tx = position.tx;
        let ty = position.ty;
        let (dx, dy) = dir.delta();
        let tp = TilePoint::new(tx + dx, ty + dy);
        let tile = self.get_tile(&tp);
        if tile.walkable() {
            Some(tp)
        } else {
            None
        }
    }

    fn is_junction(&self, tile: &TilePoint) -> bool {
        if let Some(num) = self.tile_id(tile) {
            self.junctions.contains(&num)
        } else {
            false
        }
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
                            self.chase_junctions.insert(y * self.width + x);
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
                self.chase_junctions.contains(&source),
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

        let triggers_chase = chase_change
            && self
                .boxes
                .iter()
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
                    score_change.junctions = Some((*start, *end));
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
        let val = &mut self.tiles[tile.ty as usize][tile.tx as usize];
        if *val == Tile::Painted {
            false
        } else {
            *val = Tile::Painted;
            true
        }
    }
    pub fn make_enemy(&self, ai: MovementAI, speed: i32) -> Mob {
        let fake = TilePoint::new(0, 0);
        let mut m = Mob::new(ai, fake.to_world(), speed);
        m.reset(&fake, self);
        m
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

    pub fn board_complete(&self) -> bool {
        // if this is too slow, we can store a private variable for the number of
        // unpainted tiles
        for row in &self.tiles {
            for tile in row {
                if tile.needs_paint() {
                    return false;
                }
            }
        }
        return true;
    }
}

impl State {
    pub fn try_new(config: &Amidar) -> Result<State, String> {
        let board = Board::try_new(&config.board)?;
        let mut config = config.clone();

        let enemies = config
            .enemies
            .iter()
            .map(|ai| board.make_enemy(ai.clone(), config.enemy_starting_speed))
            .collect();
        let player = Mob::new_player(config.player_start.to_world(), config.player_speed);

        let core = StateCore {
            rand: random::Gen::new_child(&mut config.rand),
            lives: config.start_lives,
            score: 0,
            chase_timer: 0,
            jumps: config.start_jumps,
            jump_timer: 0,
            level: 1,
            player,
            enemies,
            board,
        };

        let mut state = State {
            config,
            state: core,
        };
        state.reset();
        Ok(state)
    }
    pub fn reset(&mut self) {
        self.state
            .player
            .reset(&self.config.player_start, &self.state.board);
        // On the default board, we imagine starting from below the initial place.
        // This way going up paints the first segment.
        if self.config.default_board_bugs {
            self.state.player.history.push_front(
                self.state
                    .board
                    .get_junction_id(&TilePoint::new(31, 18))
                    .unwrap(),
            );
        }
        for enemy in &mut self.state.enemies {
            enemy.reset(&self.config.player_start, &self.state.board);
        }
    }
    pub fn board_size(&self) -> WorldPoint {
        let th = self.state.board.height as i32;
        let tw = self.state.board.width as i32;
        TilePoint::new(tw + 1, th + 1).to_world()
    }
    /// Determine whether an enemy and a player are colliding and what to do about it.
    /// returns: (player_dead, enemy_caught)
    fn check_enemy_player_collision(&self, enemy: &Mob, enemy_id: usize) -> EnemyPlayerState {
        if self.state.player.position.to_tile() == enemy.position.to_tile() {
            if self.state.chase_timer > 0 {
                if !enemy.caught {
                    EnemyPlayerState::EnemyCatch(enemy_id)
                } else {
                    EnemyPlayerState::Miss
                }
            } else if self.state.jump_timer > 0 {
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

impl toybox_core::Simulation for Amidar {
    fn reset_seed(&mut self, seed: u32) {
        self.rand.reset_seed(seed)
    }
    fn game_size(&self) -> (i32, i32) {
        screen::GAME_SIZE
    }
    fn new_game(&mut self) -> Box<toybox_core::State> {
        Box::new(State::try_new(self).expect("new_game should succeed."))
    }
    fn to_json(&self) -> String {
        serde_json::to_string(self).expect("Amidar should be JSON serializable!")
    }
    /// Sync with [ALE impl](https://github.com/mgbellemare/Arcade-Learning-Environment/blob/master/src/games/supported/Amidar.cpp#L80)
    /// Note, leaving a call to sort in this impl to remind users that these vecs are ordered!
    fn legal_action_set(&self) -> Vec<AleAction> {
        let mut actions = vec![
            AleAction::NOOP,
            AleAction::FIRE,
            AleAction::UP,
            AleAction::RIGHT,
            AleAction::LEFT,
            AleAction::DOWN,
            AleAction::UPFIRE,
            AleAction::RIGHTFIRE,
            AleAction::LEFTFIRE,
            AleAction::DOWNFIRE,
        ];
        actions.sort();
        actions
    }

    fn new_state_from_json(
        &self,
        json_str: &str,
    ) -> Result<Box<toybox_core::State>, serde_json::Error> {
        let state: StateCore = serde_json::from_str(json_str)?;
        Ok(Box::new(State {
            config: self.clone(),
            state,
        }))
    }

    fn from_json(
        &self,
        json_config: &str,
    ) -> Result<Box<toybox_core::Simulation>, serde_json::Error> {
        let config: Amidar = serde_json::from_str(json_config)?;
        Ok(Box::new(config))
    }
}

impl toybox_core::State for State {
    fn lives(&self) -> i32 {
        self.state.lives
    }
    fn score(&self) -> i32 {
        self.state.score
    }
    fn update_mut(&mut self, buttons: Input) {
        let pre_update_score: i32 = self.score();
        let history_limit = self.config.history_limit;

        // Move the player and determine whether the board changes.
        if let Some(score_change) = self.state.player.update(
            buttons,
            &mut self.state.board,
            None,
            history_limit,
            &mut self.state.rand,
        ) {
            // Don't award score for the first, semi-painted segment on a default Amidar board, but do paint it.
            let mut allow_score_change = true;
            if self.config.default_board_bugs {
                let (start, end) = score_change.junctions.unwrap();
                // Locations of the first, semi-painted segment.
                if start == 607 && end == 415 {
                    allow_score_change = false;
                }
            }
            if allow_score_change {
                self.state.score += score_change.horizontal;
                // max 1 point for vertical, for some reason.
                self.state.score += score_change.vertical.signum();
                self.state.score += self.config.box_bonus * score_change.num_boxes;
            }

            if score_change.triggers_chase {
                self.state.chase_timer = self.config.chase_time;
            }
        }

        if self.state.chase_timer > 0 {
            self.state.chase_timer -= 1;
        } else if self.state.jump_timer > 0 {
            // only support jump when not chasing.
            self.state.jump_timer -= 1;
        } else if (buttons.button1 || buttons.button2) && self.state.jumps > 0 {
            self.state.jump_timer = self.config.jump_time;
            self.state.jumps -= 1;
        }

        let mut dead = false;
        let mut changes: Vec<EnemyPlayerState> = Vec::new();

        // check-collisions after player move:
        for (i, e) in self.state.enemies.iter().enumerate() {
            let state = self.check_enemy_player_collision(e, i);
            if state != EnemyPlayerState::Miss {
                changes.push(state);
            }
        }

        // move enemies:
        for e in self.state.enemies.iter_mut() {
            e.update(
                Input::default(),
                &mut self.state.board,
                Some(self.state.player.clone()),
                history_limit,
                &mut self.state.rand,
            );
        }

        // check-collisions again (so we can't run through enemies):
        for (i, e) in self.state.enemies.iter().enumerate() {
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
                }
                EnemyPlayerState::PlayerDeath => {
                    dead = true;
                    break;
                }
                EnemyPlayerState::EnemyCatch(eid) => {
                    if !self.state.enemies[eid].caught {
                        self.state.score += self.config.chase_score_bonus;
                        self.state.enemies[eid].caught = true;
                    }
                }
            }
        }

        // If dead, reset. If alive, check to see if we have advanced to the next level.
        if dead {
            self.state.jumps = self.config.start_jumps;
            self.state.lives -= 1;
            self.state.score = pre_update_score;
            self.reset();
        } else {
            if self.state.board.board_complete() {
                self.reset();
                // Increment the level
                self.state.level += 1;
                // If we triggered the chase counter immediately before
                // advancing, it will still be on and will mess up the sprites. Reset to 0.
                self.state.chase_timer = 0;
                // Time to paint again!
                self.state.board = Board::fast_new();
                // If you successfully complete a level, you can get a life back (up the maximum)
                if self.lives() < self.config.start_lives {
                    self.state.lives += 1;
                }
                if self.state.level > 2 {
                    // Starting at level 3, there are six enemies.
                    // We haven't observed an agent that can get to level 3 and can't find any description
                    // of what level 3 looks like, so we are leaving this blank for now.
                }
                // Increase enemy speed.
                // Make pretty later
                let new_speed = {
                    if self.state.level < 3 {
                        self.config.enemy_starting_speed
                    } else if self.state.level < 5 {
                        self.config.enemy_starting_speed + 2
                    } else {
                        self.config.enemy_starting_speed + 4
                    }
                };
                for e in &mut self.state.enemies {
                    e.change_speed(new_speed);
                }
            }
        }
    }

    fn draw(&self) -> Vec<Drawable> {
        let mut output = Vec::new();
        output.push(Drawable::Clear(self.config.bg_color));
        if self.state.lives < 0 {
            return output;
        }

        let (tile_w, tile_h) = screen::TILE_SIZE;
        let (offset_x, offset_y) = screen::BOARD_OFFSET;

        for (ty, row) in self.state.board.tiles.iter().enumerate() {
            let ty = ty as i32;
            for (tx, tile) in row.iter().enumerate() {
                let tx = tx as i32;

                // Use the level-1 sprites for odd levels less than the sixth level.
                // Use the level-2 sprites for even levels and those greater than the sixth level.
                // We will probably want to put some of this in the config later.
                let ghosts = self.state.level % 2 == 1 && self.state.level < 6;

                if self.config.render_images {
                    let tile_sprite: &FixedSpriteData = match tile {
                        &Tile::Painted => {
                            if ghosts {
                                &images::BLOCK_TILE_PAINTED_L1
                            } else {
                                &images::BLOCK_TILE_PAINTED_L2
                            }
                        }
                        &Tile::Unpainted | &Tile::ChaseMarker => {
                            if ghosts {
                                &images::BLOCK_TILE_UNPAINTED_L1
                            } else {
                                &images::BLOCK_TILE_UNPAINTED_L2
                            }
                        }
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

        for inner_box in self.state.board.boxes.iter().filter(|b| b.painted) {
            if self.config.render_images {
                let top_left_in = inner_box.top_left.translate(1, 1);
                let x1 = top_left_in.tx;
                let x2 = inner_box.bottom_right.tx;
                let y1 = top_left_in.ty;
                let y2 = inner_box.bottom_right.ty;

                // generate all boxes inside:
                for x in x1..x2 {
                    for y in y1..y2 {
                        let pt = TilePoint::new(x, y).to_world().to_screen();
                        output.push(Drawable::sprite(
                            pt.sx + offset_x,
                            pt.sy + offset_y,
                            images::PAINTED_BOX_BAR.clone(),
                        ));
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

        let (player_x, player_y) = self.state.player.position.to_screen().pixels();
        let (player_w, player_h) = screen::PLAYER_SIZE;
        let player_sprite = match self.state.level % 2 {
            1 => images::PLAYER_L1.clone(),
            0 => images::PLAYER_L2.clone(),
            _ => unreachable!(),
        };
        if self.config.render_images {
            output.push(Drawable::sprite(
                offset_x + player_x - 1,
                offset_y + player_y - 1,
                player_sprite,
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

        for enemy in &self.state.enemies {
            let (x, y) = enemy.position.to_screen().pixels();
            let (w, h) = screen::ENEMY_SIZE;

            if self.config.render_images {
                output.push(Drawable::sprite(
                    offset_x + x - 1,
                    offset_y + y - 1,
                    if self.state.chase_timer > 0 {
                        if enemy.caught {
                            match self.state.level % 2 {
                                1 => images::ENEMY_CAUGHT_L1.clone(),
                                0 => images::ENEMY_CAUGHT_L2.clone(),
                                _ => unreachable!(),
                            }
                        } else {
                            match self.state.level % 2 {
                                1 => images::ENEMY_CHASE_L1.clone(),
                                0 => images::ENEMY_CHASE_L2.clone(),
                                _ => unreachable!(),
                            }
                        }
                    } else if self.state.jump_timer > 0 {
                        match self.state.level % 2 {
                            1 => images::ENEMY_JUMP_L1.clone(),
                            0 => images::ENEMY_JUMP_L2.clone(),
                            _ => unreachable!(),
                        }
                    } else {
                        match self.state.level % 2 {
                            1 => images::ENEMY_L1.clone(),
                            0 => images::ENEMY_L2.clone(),
                            _ => unreachable!(),
                        }
                    },
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

        output.extend(draw_score(
            self.state.score,
            screen::SCORE_X_POS,
            screen::SCORE_Y_POS + 1,
        ));
        for i in 0..self.state.lives {
            output.push(Drawable::rect(
                self.config.player_color,
                screen::LIVES_X_POS - i * screen::LIVES_X_STEP,
                screen::LIVES_Y_POS,
                1,
                DIGIT_HEIGHT + 1,
            ))
        }

        output
    }

    fn to_json(&self) -> String {
        serde_json::to_string(&self.state).expect("Should be no JSON Serialization Errors.")
    }

    fn query_json(&self, query: &str, args: &serde_json::Value) -> Result<String, QueryError> {
        let state = &self.state;
        Ok(match query {
            "world_to_tile" => {
                let world_pt: WorldPoint = serde_json::from_value(args.clone())?;
                let tile = world_pt.to_tile();
                serde_json::to_string(&(tile.tx, tile.ty))?
            }
            "tile_to_world" => {
                let tile_pt: TilePoint = serde_json::from_value(args.clone())?;
                let world = tile_pt.to_world();
                serde_json::to_string(&(world.x, world.y))?
            }
            "num_tiles_unpainted" => {
                let mut sum = 0;
                for row in state.board.tiles.iter() {
                    sum += row
                        .iter()
                        .filter(|t| t.walkable() && t.needs_paint())
                        .count();
                }
                serde_json::to_string(&sum)?
            }
            "regular_mode" => {
                serde_json::to_string(&(state.chase_timer == 0 && state.jump_timer == 0))?
            }
            "jump_mode" => serde_json::to_string(&(state.jump_timer > 0))?,
            "chase_mode" => serde_json::to_string(&(state.chase_timer > 0))?,
            "jumps_remaining" => serde_json::to_string(&(state.jumps > 0))?,
            "num_enemies" => serde_json::to_string(&state.enemies.len())?,
            "enemy_tiles" => {
                let positions: Vec<(i32, i32)> = state
                    .enemies
                    .iter()
                    .map(|e| {
                        let tile = e.position.to_tile();
                        (tile.tx, tile.ty)
                    })
                    .collect();
                serde_json::to_string(&positions)?
            }
            "enemy_tile" => {
                if let Some(index) = args.as_u64() {
                    let tile = state.enemies[index as usize].position.to_tile();
                    serde_json::to_string(&(tile.tx, tile.ty))?
                } else {
                    Err(QueryError::BadInputArg)?
                }
            }
            "enemy_caught" => {
                if let Some(index) = args.as_u64() {
                    let status = state.enemies[index as usize].caught;
                    serde_json::to_string(&status)?
                } else {
                    Err(QueryError::BadInputArg)?
                }
            }
            "player_tile" => {
                let tile = state.player.position.to_tile();
                serde_json::to_string(&(tile.tx, tile.ty))?
            }
            _ => Err(QueryError::NoSuchQuery)?,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use toybox_core::State;

    #[test]
    fn test_colors_unique_in_gray() {
        let config = Amidar::default();
        let num_colors = config.colors().len();
        let uniq_grays: HashSet<u8> = config
            .colors()
            .into_iter()
            .map(|c| c.grayscale_byte())
            .collect();
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
        assert!(img.width() > 0);
        assert!(img.height() > 0);
        assert!(img.find_visible_color().is_some());
    }

    #[test]
    fn what_json_do_we_want() {
        let data = MovementAI::EnemyPerimeterAI {
            start: TilePoint::new(0, 0),
        };
        println!("{}", serde_json::to_string_pretty(&data).unwrap());
        let data = MovementAI::EnemyTargetPlayer {
            vision_distance: 15,
            start: TilePoint::new(0, 0),
            start_dir: Direction::Up,
            dir: Direction::Up,
            player_seen: None,
        };
        println!("{}", serde_json::to_string_pretty(&data).unwrap());
    }

    fn player_tile(state: &State) -> (i32, i32) {
        serde_json::from_str(
            &state
                .query_json("player_tile", &serde_json::Value::Null)
                .unwrap(),
        )
        .unwrap()
    }
    fn num_tiles_unpainted(state: &State) -> usize {
        serde_json::from_str(
            &state
                .query_json("num_tiles_unpainted", &serde_json::Value::Null)
                .unwrap(),
        )
        .unwrap()
    }

    #[test]
    fn test_q_num_tiles_unpainted() {
        let mut state = super::State::try_new(&Amidar::default()).unwrap();

        let (px, py) = player_tile(&state);
        let first = num_tiles_unpainted(&state);

        let mut go_up = Input::default();
        go_up.up = true;

        // Move the user up (be a little robust to how long the animation takes.)
        for _ in 0..5000 {
            state.update_mut(go_up);
            if state.score() > 0 {
                // we must have painted something!
                break;
            }
        }

        let (nx, ny) = player_tile(&state);
        if py == ny {
            panic!("Player can't move upward!")
        }
        println!("Moved player to ({},{}) from ({},{})", nx, ny, px, py);

        let painted_now = num_tiles_unpainted(&state);
        println!("painted_now: {} ... before: {}", painted_now, first);
        assert!(painted_now < first);
    }
}
