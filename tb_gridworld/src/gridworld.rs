use rand::Rng;
use rand_core::RngCore;
use toybox_core::graphics::{Color, Drawable};
use toybox_core::random;
use toybox_core::{AleAction, Direction, Input, QueryError};

use types::{DiagonalDir, Enemy, FrameState, GridWorld, State, TileBehavior};

use serde_json;
use std::collections::HashMap;

impl Default for GridWorld {
    /// The default game features as many behaviors as possible!
    fn default() -> Self {
        let mut tiles = HashMap::new();
        tiles.insert('1', TileBehavior::Wall);
        tiles.insert('0', TileBehavior::Floor);
        tiles.insert('R', TileBehavior::ReceiveReward(1));
        tiles.insert('G', TileBehavior::WinGame);
        tiles.insert('D', TileBehavior::LoseGame);
        tiles.insert(
            'H',
            TileBehavior::MaybeLoseGame(0.5, Color::rgb(100, 100, 100)),
        );
        let door_color = Color::rgb(150, 150, 150);
        tiles.insert(
            'X',
            TileBehavior::Door {
                switch_id: 1,
                open: door_color,
                closed: door_color,
            },
        );
        tiles.insert(
            'Y',
            TileBehavior::DoorSwitch {
                switch_id: 1,
                state: false,
                on_color: Color::rgb(150, 250, 150),
                off_color: Color::rgb(250, 150, 150),
            },
        );

        let grid = vec![
            "111111111".to_owned(), // 0
            "1000R0001".to_owned(), // 1
            "1X1111101".to_owned(), // 2
            "100Y10001".to_owned(), // 3
            "100010001".to_owned(), // 4
            "100010001".to_owned(), // 5
            "10001R111".to_owned(), // 6
            "100D10HG1".to_owned(), // 7
            "111111111".to_owned(), // 8
        ];

        GridWorld {
            player_start: (2, 7),
            grid,
            tiles,
            diagonal_support: false,
            enemies: vec![Enemy {
                positions: vec![(1, 4), (2, 4), (3, 4), (2, 4)],
                current_time: 0,
                color: Color::rgb(255, 0, 255),
            }],
            rand: random::Gen::new_from_seed(42),

            // Now colors are more-or-less configurable.
            player_color: Color::rgb(255, 0, 0),
            wall_color: Color::black(),
            win_color: Color::rgb(0, 255, 0),
            lose_color: Color::rgb(255, 0, 0),
            floor_color: Color::white(),
            reward_color: Color::rgb(255, 255, 0),

            // This is probably a hyperparameter anyway, so moved to config.
            lose_reward: -100,
            win_reward: 100,
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
        let mut tile_states = Vec::new();

        for row in config.grid.iter() {
            let mut row_states = Vec::new();
            for tile in row.chars() {
                row_states.push(config.tiles[&tile].clone());
            }
            tile_states.push(row_states);
        }

        FrameState {
            game_over: false,
            step: 0,
            score: 0,
            grid: tile_states,
            player: config.player_start,
            enemies: config.enemies.clone(),
            rand: random::Gen::new_from_seed(config.rand.next_u32()),
        }
    }
    fn get_tile_mut(&mut self, tx: i32, ty: i32) -> Option<&mut TileBehavior> {
        let (w, h) = self.size();
        if tx < 0 || ty < 0 || tx >= w || ty >= h {
            return None;
        }
        self.grid
            .get_mut(ty as usize)
            .and_then(|row| row.get_mut(tx as usize))
    }
    fn get_tile(&self, tx: i32, ty: i32) -> Option<&TileBehavior> {
        let (w, h) = self.size();
        if tx < 0 || ty < 0 || tx >= w || ty >= h {
            return None;
        }
        self.grid
            .get(ty as usize)
            .and_then(|row| row.get(tx as usize))
    }
    fn tile_color(&self, tile: &TileBehavior, config: &GridWorld) -> Color {
        match tile {
            TileBehavior::DoorSwitch {
                state,
                on_color,
                off_color,
                ..
            } => {
                if *state {
                    *on_color
                } else {
                    *off_color
                }
            }
            TileBehavior::ReceiveReward { .. } => config.reward_color,
            TileBehavior::LoseGame => config.lose_color,
            TileBehavior::WinGame => config.win_color,
            TileBehavior::MaybeLoseGame(_, color) => *color,
            TileBehavior::Wall => config.wall_color,
            TileBehavior::Floor => config.floor_color,
            TileBehavior::Door {
                switch_id,
                open,
                closed,
            } => {
                if self.get_switch_state(*switch_id) {
                    open.clone()
                } else {
                    closed.clone()
                }
            }
        }
    }
    fn get_switch_state(&self, of_switch_id: u32) -> bool {
        for row in self.grid.iter() {
            for tile in row.iter() {
                if let TileBehavior::DoorSwitch {
                    switch_id, state, ..
                } = tile
                {
                    if *switch_id == of_switch_id {
                        return *state;
                    }
                }
            }
        }
        panic!("switch_state({}) switch does not exist!", of_switch_id);
    }
    fn walkable(&self, tx: i32, ty: i32) -> bool {
        if let Some(behavior) = self.get_tile(tx, ty) {
            match behavior {
                TileBehavior::LoseGame
                | TileBehavior::WinGame
                | TileBehavior::MaybeLoseGame(_, _) => true,
                TileBehavior::DoorSwitch { .. } => true,
                TileBehavior::ReceiveReward { .. } => true,
                TileBehavior::Wall => false,
                TileBehavior::Floor => true,
                TileBehavior::Door { switch_id, .. } => self.get_switch_state(*switch_id),
            }
        } else {
            false
        }
    }
    /// Take a step if the destination is walkable.
    fn walk_once(&mut self, dx: i32, dy: i32, config: &GridWorld) {
        let (px, py) = self.player;
        let dest = (px + dx, py + dy);
        if self.walkable(dest.0, dest.1) {
            self.arrive(dest.0, dest.1, config)
        }
    }

    /// Can move up and left (Northwest?)
    /// No No Yes Yes Yes
    /// XX .X ..  .X  ..
    /// X@ X@ X@  .@  .@
    ///
    /// Or in words: you can move diagonally if the destination is free AND you are not blocked on both vertical and horizontal roads.
    fn walk_diagonal(&mut self, dx: i32, dy: i32, config: &GridWorld) {
        let (px, py) = self.player;

        if self.walkable(px + dx, py + dy)
            && (self.walkable(px + dx, py) || self.walkable(px, py + dy))
        {
            self.arrive(px + dx, py + dy, config)
        }
    }

    /// Move to a new location.
    fn arrive(&mut self, x: i32, y: i32, config: &GridWorld) {
        self.player = (x, y);

        // There should be a tile here for you to arrive!
        match self.get_tile(x, y).unwrap().clone() {
            TileBehavior::Wall | TileBehavior::Floor | TileBehavior::Door { .. } => {}
            TileBehavior::LoseGame => {
                self.score += config.lose_reward;
                self.game_over = true;
            }
            TileBehavior::WinGame => {
                self.score += config.win_reward;
                self.game_over = true;
            }
            TileBehavior::MaybeLoseGame(terminal, _) => {
                let p: f64 = self.rand.gen_range(0.0, 1.0);
                println!("p={}, terminal={}", p, terminal);
                if p < terminal {
                    self.game_over = true;
                    self.score += config.lose_reward;
                }
            }
            TileBehavior::ReceiveReward(amt) => {
                self.score += amt;
                *self.get_tile_mut(x, y).unwrap() = TileBehavior::Floor;
            }
            TileBehavior::DoorSwitch { state, .. } => {
                let orig_state = state;
                if let TileBehavior::DoorSwitch { ref mut state, .. } =
                    self.get_tile_mut(x, y).unwrap()
                {
                    *state = !orig_state;
                }
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
                    DiagonalDir::N => self.frame.walk_once(0, -1, &self.config),
                    DiagonalDir::S => self.frame.walk_once(0, 1, &self.config),
                    DiagonalDir::E => self.frame.walk_once(1, 0, &self.config),
                    DiagonalDir::W => self.frame.walk_once(-1, 0, &self.config),
                    DiagonalDir::NW => self.frame.walk_diagonal(-1, -1, &self.config),
                    DiagonalDir::NE => self.frame.walk_diagonal(1, -1, &self.config),
                    DiagonalDir::SW => self.frame.walk_diagonal(-1, 1, &self.config),
                    DiagonalDir::SE => self.frame.walk_diagonal(1, 1, &self.config),
                }
            }
        } else {
            if let Some(dir) = Direction::from_input(buttons) {
                let (dx, dy) = dir.delta();
                self.frame.walk_once(dx, dy, &self.config);
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
                output.push(Drawable::rect(
                    self.frame.tile_color(tile, &self.config),
                    x as i32,
                    y as i32,
                    1,
                    1,
                ));
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
