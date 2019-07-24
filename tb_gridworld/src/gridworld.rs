use toybox_core::graphics::{Color, Drawable};
use toybox_core::{AleAction, Direction, Input, QueryError};

use types::{DiagonalDir, FrameState, GridWorld, State, TileConfig};

use serde_json;
use std::collections::HashMap;

impl TileConfig {
    fn wall() -> TileConfig {
        TileConfig {
            reward: 0,
            walkable: false,
            terminal: false,
            color: Color::black(),
        }
    }
    fn floor() -> TileConfig {
        TileConfig {
            reward: 0,
            walkable: true,
            terminal: false,
            color: Color::white(),
        }
    }
    fn reward() -> TileConfig {
        TileConfig {
            reward: 1,
            walkable: true,
            terminal: false,
            color: Color::rgb(255, 255, 0),
        }
    }
    fn goal() -> TileConfig {
        TileConfig {
            reward: 10,
            walkable: true,
            terminal: true,
            color: Color::rgb(0, 255, 0),
        }
    }
    fn death() -> TileConfig {
        TileConfig {
            reward: -10,
            walkable: true,
            terminal: true,
            color: Color::rgb(255, 0, 0),
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

        let grid = vec![
            "111111111".to_owned(),
            "1000R0001".to_owned(),
            "101111101".to_owned(),
            "100010001".to_owned(),
            "10001R111".to_owned(),
            "100D100G1".to_owned(),
            "111111111".to_owned(),
        ];

        GridWorld {
            player_color: Color::rgb(255, 0, 0),
            player_start: (2, 4),
            reward_becomes: '0',
            grid,
            tiles,
            diagonal_support: false,
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
    fn from_config(config: &GridWorld) -> FrameState {
        let mut tiles = Vec::new();
        let mut grid = Vec::new();

        let mut char_to_index = HashMap::new();
        for (ch, desc) in &config.tiles {
            let id = tiles.len();
            char_to_index.insert(ch, id);
            tiles.push(desc.clone());
        }
        for row in &config.grid {
            let mut grid_row = Vec::new();
            for ch in row.chars() {
                let tile_id = char_to_index[&ch];
                grid_row.push(tile_id);
            }
            grid.push(grid_row);
        }

        FrameState {
            game_over: false,
            step: 0,
            score: 0,
            reward_becomes: char_to_index[&config.reward_becomes],
            tiles,
            grid,
            player: config.player_start,
        }
    }
    fn get_tile(&self, tx: i32, ty: i32) -> Option<&TileConfig> {
        let (w, h) = self.size();
        if tx < 0 || ty < 0 || tx >= w || ty >= h {
            return None;
        }
        let y = ty as usize;
        let x = tx as usize;
        let tile_id = self.grid[y][x];
        Some(&self.tiles[tile_id])
    }
    fn walkable(&self, tx: i32, ty: i32) -> bool {
        self.get_tile(tx, ty).map(|t| t.walkable).unwrap_or(false)
    }
    fn terminal(&self, tx: i32, ty: i32) -> bool {
        self.get_tile(tx, ty).map(|t| t.terminal).unwrap_or(false)
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

    fn collect_reward(&mut self, tx: i32, ty: i32) -> i32 {
        let y = ty as usize;
        let x = tx as usize;
        let tile_id = self.grid[y][x];
        let reward = self.tiles[tile_id].reward;
        if reward != 0 {
            self.grid[y][x] = self.reward_becomes;
        }
        reward
    }
    /// Move to a new location.
    fn arrive(&mut self, x: i32, y: i32) {
        self.player = (x, y);

        // check terminal before "collect_reward" which removes the reward from the map.
        if self.terminal(x, y) {
            self.game_over = true;
        }

        self.collect_reward(x, y);
    }
}

impl toybox_core::Simulation for GridWorld {
    fn reset_seed(&mut self, _seed: u32) {}

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
            frame: FrameState::from_config(&self),
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
            0
        } else {
            1
        }
    }
    fn score(&self) -> i32 {
        self.frame.score
    }

    fn update_mut(&mut self, buttons: Input) {
        // Must take an action in GridWorld.
        if buttons.is_empty() {
            return;
        }
        self.frame.step += 1;

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
    }
    fn draw(&self) -> Vec<Drawable> {
        let mut output = Vec::new();
        output.push(Drawable::Clear(Color::black()));

        let (width, height) = self.frame.size();
        for y in 0..height {
            for x in 0..width {
                let tile = self.frame.get_tile(x, y).expect("Tile type should exist!");
                output.push(Drawable::rect(tile.color, x as i32, y as i32, 1, 1));
            }
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
