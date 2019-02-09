use toybox_core::graphics::{Color, Drawable};
use toybox_core::{AleAction, Direction, Input, QueryError};

use serde_json;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TileConfig {
    /// What reward (if any) is given or taken by passing this tile?
    pub reward: i32,
    /// Is this tile walkable by the agent?
    pub walkable: bool,
    /// Is this a terminal/goal tile?
    pub goal: bool,
    /// What color should this tile be?
    pub color: Color,
}

impl TileConfig {
    fn wall() -> TileConfig {
        TileConfig {
            reward: 0,
            walkable: false,
            goal: false,
            color: Color::black(),
        }
    }
    fn floor() -> TileConfig {
        TileConfig {
            reward: 0,
            walkable: true,
            goal: false,
            color: Color::white(),
        }
    }
    fn reward() -> TileConfig {
        TileConfig {
            reward: 1,
            walkable: true,
            goal: false,
            color: Color::rgb(255, 255, 0),
        }
    }
    fn goal() -> TileConfig {
        TileConfig {
            reward: 10,
            walkable: true,
            goal: true,
            color: Color::rgb(0, 255, 0),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GridWorld {
    pub game_size: (i32, i32),
    pub grid: Vec<String>,
    pub tiles: HashMap<char, TileConfig>,
    pub reward_becomes: char,
    pub player_color: Color,
    pub player_start: (i32, i32),
}

impl Default for GridWorld {
    fn default() -> Self {
        let mut tiles = HashMap::new();
        tiles.insert('1', TileConfig::wall());
        tiles.insert('0', TileConfig::floor());
        tiles.insert('R', TileConfig::reward());
        tiles.insert('G', TileConfig::goal());

        let grid = vec![
            "111111111".to_owned(),
            "1000R0001".to_owned(),
            "101111101".to_owned(),
            "100010001".to_owned(),
            "10001R111".to_owned(),
            "1000100G1".to_owned(),
            "111111111".to_owned(),
        ];

        let width = grid[0].len() as i32;
        let height = grid.len() as i32;
        GridWorld {
            game_size: (width, height),
            player_color: Color::rgb(255, 0, 0),
            player_start: (2, 4),
            reward_becomes: '0',
            grid,
            tiles,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct State {
    pub game_over: bool,
    pub score: i32,
    pub reward_becomes: usize,
    pub tiles: Vec<TileConfig>,
    pub grid: Vec<Vec<usize>>,
    pub player: (i32, i32),
    pub player_color: Color,
}
impl State {
    fn size(&self) -> (i32, i32) {
        let height = self.grid.len() as i32;
        let width = self.grid[0].len() as i32;
        (width, height)
    }
    fn from_config(config: &GridWorld) -> State {
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

        State {
            game_over: false,
            score: 0,
            reward_becomes: char_to_index[&config.reward_becomes],
            tiles,
            grid,
            player_color: config.player_color,
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
}

impl toybox_core::Simulation for GridWorld {
    fn reset_seed(&mut self, _seed: u32) {}
    fn game_size(&self) -> (i32, i32) {
        self.game_size
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
        Box::new(State::from_config(&self))
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

impl toybox_core::State for State {
    fn lives(&self) -> i32 {
        if self.game_over {
            1
        } else {
            0
        }
    }
    fn score(&self) -> i32 {
        self.score
    }
    fn update_mut(&mut self, buttons: Input) {
        // Must take an action in GridWorld.
        if buttons.is_empty() {
            return;
        }
        if let Some(dir) = Direction::from_input(buttons) {
            let (dx, dy) = dir.delta();
            let (px, py) = self.player;
            let dest = (px + dx, py + dy);

            if self.walkable(dest.0, dest.1) {
                self.player = dest;
                self.collect_reward(dest.0, dest.1);
            }
        }
    }
    fn draw(&self) -> Vec<Drawable> {
        let mut output = Vec::new();
        output.push(Drawable::Clear(Color::black()));

        for (y, row) in self.grid.iter().enumerate() {
            for (x, cell) in row.iter().enumerate() {
                let tile = &self.tiles[*cell];
                output.push(Drawable::rect(tile.color, x as i32, y as i32, 1, 1));
            }
        }
        output.push(Drawable::rect(
            self.player_color,
            self.player.0,
            self.player.1,
            1,
            1,
        ));

        output
    }
    fn to_json(&self) -> String {
        serde_json::to_string(self).expect("Should be no JSON Serialization Errors.")
    }

    fn query_json(&self, _query: &str, _args: &serde_json::Value) -> Result<String, QueryError> {
        Err(QueryError::NoSuchQuery)
    }
}
