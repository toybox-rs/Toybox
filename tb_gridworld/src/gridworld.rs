use rand::Rng;
use rand_core::RngCore;
use toybox_core::graphics::{Color, Drawable};
use toybox_core::random;
use toybox_core::{AleAction, Direction, Input, QueryError};

use types::{DiagonalDir, Enemy, FrameState, GridWorld, State, TileConfig};

use serde_json;
use std::collections::HashMap;

impl TileConfig {
    fn wall() -> TileConfig {
        TileConfig {
            reward: 0,
            walkable: false,
            terminal: 0.0,
            color: Color::black(),
        }
    }
    fn floor() -> TileConfig {
        TileConfig {
            reward: 0,
            walkable: true,
            terminal: 0.0,
            color: Color::white(),
        }
    }
    fn reward() -> TileConfig {
        TileConfig {
            reward: 1,
            walkable: true,
            terminal: 0.0,
            color: Color::rgb(255, 255, 0),
        }
    }
    fn goal() -> TileConfig {
        TileConfig {
            reward: 10,
            walkable: true,
            terminal: 1.0,
            color: Color::rgb(0, 255, 0),
        }
    }
    fn death() -> TileConfig {
        TileConfig {
            reward: -10,
            walkable: true,
            terminal: 1.0,
            color: Color::rgb(255, 0, 0),
        }
    }
    fn half_death() -> TileConfig {
        TileConfig {
            reward: -10,
            walkable: true,
            terminal: 0.5,
            color: Color::rgb(100, 100, 100),
        }
    }
}

impl Default for GridWorld {
    fn default() -> Self {
        let mut tiles = HashMap::new();
        tiles.insert('1', TileConfig::wall());
        tiles.insert('0', TileConfig::floor());
        tiles.insert('R', TileConfig::reward());
        tiles.insert('G', TileConfig::goal());
        tiles.insert('D', TileConfig::death());
        tiles.insert('H', TileConfig::half_death());

        let grid = vec![
            "111111111".to_owned(), // 0
            "1000R0001".to_owned(), // 1
            "101111101".to_owned(), // 2
            "100010001".to_owned(), // 3
            "100010001".to_owned(), // 4
            "100010001".to_owned(), // 5
            "10001R111".to_owned(), // 6
            "100D10HG1".to_owned(), // 7
            "111111111".to_owned(), // 8
        ];

        GridWorld {
            player_color: Color::rgb(255, 0, 0),
            player_start: (2, 7),
            reward_becomes: '0',
            grid,
            tiles,
            diagonal_support: false,
            enemies: vec![Enemy {
                positions: vec![(1, 4), (2, 4), (3, 4), (2, 4)],
                current_time: 0,
                color: Color::rgb(255, 0, 255),
            }],
            rand: random::Gen::new_from_seed(42),
        }
    }
}

impl FrameState {
    /// Compute the size of the grid for our own usage here.
    fn size(&self) -> (i32, i32) {
        let height = self.grid.len() as i32;
        let width = self.grid[0].len() as i32;
        (width, height)
    }
    fn from_config(config: &mut GridWorld) -> FrameState {
        FrameState {
            game_over: false,
            step: 0,
            score: 0,
            reward_becomes: config.reward_becomes,
            tiles: config.tiles.clone(),
            grid: config.grid.clone(),
            player: config.player_start,
            enemies: config.enemies.clone(),
            rand: random::Gen::new_from_seed(config.rand.next_u32()),
        }
    }
    fn get_tile(&self, tx: i32, ty: i32) -> Option<TileConfig> {
        let (w, h) = self.size();
        if tx < 0 || ty < 0 || tx >= w || ty >= h {
            return None;
        }
        let tile_id = self.get_tile_id(tx, ty);
        Some(self.tiles[&tile_id].clone())
    }
    fn walkable(&self, tx: i32, ty: i32) -> bool {
        self.get_tile(tx, ty).map(|t| t.walkable).unwrap_or(false)
    }
    fn terminal(&self, tx: i32, ty: i32) -> f64 {
        self.get_tile(tx, ty).map(|t| t.terminal).unwrap_or(0.0)
    }
    /// Take a step if the destination is walkable.
    fn walk_once(&mut self, dx: i32, dy: i32) {
        let (px, py) = self.player;
        let dest = (px + dx, py + dy);
        if self.walkable(dest.0, dest.1) {
            self.arrive(dest.0, dest.1)
        }
    }

    /// Can move up and left (Northwest?)
    /// No No Yes Yes Yes
    /// XX .X ..  .X  ..
    /// X@ X@ X@  .@  .@
    ///
    /// Or in words: you can move diagonally if the destination is free AND you are not blocked on both vertical and horizontal roads.
    fn walk_diagonal(&mut self, dx: i32, dy: i32) {
        let (px, py) = self.player;

        if self.walkable(px + dx, py + dy)
            && (self.walkable(px + dx, py) || self.walkable(px, py + dy))
        {
            self.arrive(px + dx, py + dy)
        }
    }

    fn get_tile_id(&self, tx: i32, ty: i32) -> char {
        let y = ty as usize;
        let x = tx as usize;
        self.grid[y]
            .chars()
            .nth(x)
            .expect("get_tile_id private method got bad coordinate!")
    }

    fn set_tile_id(&mut self, tx: i32, ty: i32, new_id: char) {
        let y = ty as usize;
        let x = tx as usize;
        let mut row: Vec<char> = self.grid[y].chars().collect();
        row[x] = new_id;
        self.grid[y] = row.into_iter().collect()
    }

    fn collect_reward(&mut self, tx: i32, ty: i32) -> i32 {
        let tile_id = self.get_tile_id(tx, ty);
        let reward = self.tiles[&tile_id].reward;
        if reward != 0 {
            let transition = self.reward_becomes;
            self.set_tile_id(tx, ty, transition);
        }
        reward
    }
    /// Move to a new location.
    fn arrive(&mut self, x: i32, y: i32) {
        self.player = (x, y);

        // check terminal before "collect_reward" which removes the reward from the map.
        let terminal = self.terminal(x, y);
        if terminal == 0.0 {
            self.collect_reward(x, y);
        } else if terminal == 1.0 {
            self.game_over = true;
            self.collect_reward(x, y);
        } else {
            let p: f64 = self.rand.gen_range(0.0, 1.0);
            println!("p={}, terminal={}", p, terminal);
            if p < terminal {
                self.game_over = true;
                self.collect_reward(x, y);
            }
        }
    }
    fn check_enemy_death(&mut self) {
        if self.game_over {
            return;
        }
        for e in self.enemies.iter() {
            let (ex, ey) = e.positions[e.current_time as usize];
            let (px, py) = self.player;
            if ex == px && ey == py {
                self.game_over = true;
                break;
            }
        }
    }

    fn move_enemies(&mut self) {
        for e in self.enemies.iter_mut() {
            e.current_time += 1;
            e.current_time %= e.positions.len() as u32;
        }
    }
}

impl toybox_core::Simulation for GridWorld {
    fn reset_seed(&mut self, seed: u32) {
        self.rand.reset_seed(seed);
    }

    /// Compute the size of the grid for determining how big the world should be.
    fn game_size(&self) -> (i32, i32) {
        let height = self.grid.len() as i32;
        let width = self.grid[0].len() as i32;
        (width, height)
    }

    fn legal_action_set(&self) -> Vec<AleAction> {
        let mut actions = vec![
            AleAction::NOOP,
            AleAction::LEFT,
            AleAction::RIGHT,
            AleAction::UP,
            AleAction::DOWN,
        ];
        actions.sort();
        actions
    }

    fn new_game(&mut self) -> Box<toybox_core::State> {
        Box::new(State {
            frame: FrameState::from_config(self),
            config: self.clone(),
        })
    }

    fn new_state_from_json(
        &self,
        json_str: &str,
    ) -> Result<Box<toybox_core::State>, serde_json::Error> {
        let state: State = serde_json::from_str(json_str)?;
        Ok(Box::new(state))
    }

    fn to_json(&self) -> String {
        serde_json::to_string(self).expect("GridWorld should be JSON-serializable!")
    }

    fn from_json(&self, json_str: &str) -> Result<Box<toybox_core::Simulation>, serde_json::Error> {
        let config: GridWorld = serde_json::from_str(json_str)?;
        Ok(Box::new(config))
    }
}

impl DiagonalDir {
    /// Read an input struct and turn it into a diagonal direction.
    fn from_input(buttons: Input) -> Option<DiagonalDir> {
        match (buttons.left, buttons.up, buttons.right, buttons.down) {
            (true, false, false, false) => Some(DiagonalDir::W),
            (true, true, false, false) => Some(DiagonalDir::NW),
            (false, true, false, false) => Some(DiagonalDir::N),
            (false, true, true, false) => Some(DiagonalDir::NE),
            (false, false, true, false) => Some(DiagonalDir::E),
            (false, false, true, true) => Some(DiagonalDir::SE),
            (false, false, false, true) => Some(DiagonalDir::S),
            (true, false, false, true) => Some(DiagonalDir::SW),
            _ => None,
        }
    }
}

impl toybox_core::State for State {
    fn lives(&self) -> i32 {
        if self.frame.game_over {
            -1000
        } else {
            1
        }
    }
    fn score(&self) -> i32 {
        self.frame.score
    }

    fn update_mut(&mut self, buttons: Input) {
        if self.frame.game_over {
            return;
        }
        // Must take an action in GridWorld.
        if buttons.is_empty() {
            return;
        }
        self.frame.step += 1;

        // Check enemy<->player collision.
        self.frame.check_enemy_death();
        // Move enemies.
        self.frame.move_enemies();
        // Check again.
        self.frame.check_enemy_death();

        // Move player.
        if self.config.diagonal_support {
            if let Some(ddir) = DiagonalDir::from_input(buttons) {
                match ddir {
                    DiagonalDir::N => self.frame.walk_once(0, -1),
                    DiagonalDir::S => self.frame.walk_once(0, 1),
                    DiagonalDir::E => self.frame.walk_once(1, 0),
                    DiagonalDir::W => self.frame.walk_once(-1, 0),
                    DiagonalDir::NW => self.frame.walk_diagonal(-1, -1),
                    DiagonalDir::NE => self.frame.walk_diagonal(1, -1),
                    DiagonalDir::SW => self.frame.walk_diagonal(-1, 1),
                    DiagonalDir::SE => self.frame.walk_diagonal(1, 1),
                }
            }
        } else {
            if let Some(dir) = Direction::from_input(buttons) {
                let (dx, dy) = dir.delta();
                self.frame.walk_once(dx, dy);
            }
        }
        // Check enemy<->player collision again!
        self.frame.check_enemy_death();
    }
    fn draw(&self) -> Vec<Drawable> {
        let mut output = Vec::new();
        output.push(Drawable::Clear(Color::black()));
        if self.frame.game_over {
            return output;
        }

        let (width, height) = self.frame.size();
        for y in 0..height {
            for x in 0..width {
                let tile = self.frame.get_tile(x, y).expect("Tile type should exist!");
                output.push(Drawable::rect(tile.color, x as i32, y as i32, 1, 1));
            }
        }

        for e in self.frame.enemies.iter() {
            let (ex, ey) = e.positions[e.current_time as usize];
            output.push(Drawable::rect(e.color, ex, ey, 1, 1));
        }

        output.push(Drawable::rect(
            self.config.player_color,
            self.frame.player.0,
            self.frame.player.1,
            1,
            1,
        ));

        output
    }
    fn to_json(&self) -> String {
        serde_json::to_string(self).expect("Should be no JSON Serialization Errors.")
    }

    fn query_json(&self, query: &str, _args: &serde_json::Value) -> Result<String, QueryError> {
        Ok(match query {
            "xy" => {
                let (px, py) = self.frame.player;
                serde_json::to_string(&(px, py))?
            }
            "xyt" => {
                let (px, py) = self.frame.player;
                serde_json::to_string(&(px, py, self.frame.step))?
            }
            _ => Err(QueryError::NoSuchQuery)?,
        })
    }
}
