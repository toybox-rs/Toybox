use super::destruction;
use super::font::{draw_score, get_sprite, FontChoice};
use itertools::Itertools;
use serde_json;
use std::any::Any;
use toybox_core::collision::Rect;
use toybox_core::graphics::{Color, Drawable, FixedSpriteData, SpriteData};
use toybox_core::random;
use toybox_core::{AleAction, Direction, Input};

pub mod screen {
    pub const GAME_SIZE: (i32, i32) = (320, 210);
    pub const SKY_TO_GROUND: i32 = 195;

    pub const GAME_DOT_LEFT: i32 = 66;
    pub const GAME_DOT_RIGHT: i32 = 244;
    pub const GAME_DOT_SIZE: (i32, i32) = (4, 5);
    pub const SCORE_LEFT_X_POS: i32 = 8;
    pub const SCORE_RIGHT_X_POS: i32 = 168;
    pub const SCORE_Y_POS: i32 = 8;
    pub const SHIP_SIZE: (i32, i32) = (16, 10);
    pub const SHIELD_SIZE: (i32, i32) = (16, 18);
    pub const SHIELD1_POS: (i32, i32) = (84, 157);
    pub const SHIELD2_POS: (i32, i32) = (148, 157);
    pub const SHIELD3_POS: (i32, i32) = (212, 157);

    pub const ENEMY_SIZE: (i32, i32) = (16, 10);
    pub const ENEMY_START_POS: (i32, i32) = (44, 31);
    pub const ENEMY_END_POS: (i32, i32) = (98, 31);
    pub const ENEMIES_PER_ROW: i32 = 6;
    pub const ENEMIES_NUM: i32 = 6;
    pub const ENEMY_SPACE: (i32, i32) = (16, 8);
    pub const ENEMY_DELTA: (i32, i32) = (2, 10);
    pub const ENEMY_PERIOD: i32 = 32;

    pub const NEW_LIFE_TIME: i32 = 128;

    pub const DEATH_TIME: i32 = 29;
    pub const DEATH_HIT_1: i32 = 5;
    pub const DEATH_HIT_N: i32 = 8;

    pub const UFO_SIZE: (i32, i32) = (21, 13);
    pub const LASER_SIZE_W: i32 = 2;
    pub const LASER_SIZE_H1: i32 = 11;
    pub const LASER_SIZE_H2: i32 = 8;

    // Colors:
    pub const LEFT_GAME_DOT_COLOR: (u8, u8, u8) = (64, 124, 64);
    pub const RIGHT_GAME_DOT_COLOR: (u8, u8, u8) = (160, 132, 68);
    pub const LIVES_DISPLAY_COLOR: (u8, u8, u8) = (162, 134, 56);
    pub const SHIELD_COLOR: (u8, u8, u8) = (172, 80, 48);
    pub const ENEMY_COLOR: (u8, u8, u8) = (132, 132, 36);
    pub const UFO_COLOR: (u8, u8, u8) = (140, 32, 116);
    pub const LASER_COLOR: (u8, u8, u8) = (144, 144, 144);
    pub const GROUND_COLOR: (u8, u8, u8) = (76, 80, 28);
    pub const SHIP_COLOR: (u8, u8, u8) = (35, 129, 59);

    pub const LIVES_DISPLAY_POSITION: (i32, i32) = (168, 185);

    pub const SHIP_LIMIT_X1: i32 = GAME_DOT_LEFT + GAME_DOT_SIZE.0 / 2;
    pub const SHIP_LIMIT_X2: i32 = (GAME_DOT_RIGHT + GAME_DOT_SIZE.0 / 2) - SHIP_SIZE.0;

    pub const SHIELD_SPRITE_DATA: &str = include_str!("resources/space_invader_shield_x3");

}
lazy_static! {
    static ref INVADER_INIT_1: FixedSpriteData = load_sprite_default(
        include_str!("resources/space_invaders/invader_init_1"),
        (&screen::ENEMY_COLOR).into(),
        1
    )
    .unwrap()
    .to_fixed()
    .unwrap();
    static ref INVADER_INIT_2: FixedSpriteData = load_sprite_default(
        include_str!("resources/space_invaders/invader_init_2"),
        (&screen::ENEMY_COLOR).into(),
        1
    )
    .unwrap()
    .to_fixed()
    .unwrap();
    static ref INVADER_INIT_3: FixedSpriteData = load_sprite_default(
        include_str!("resources/space_invaders/invader_init_3"),
        (&screen::ENEMY_COLOR).into(),
        1
    )
    .unwrap()
    .to_fixed()
    .unwrap();
    static ref INVADER_INIT_4: FixedSpriteData = load_sprite_default(
        include_str!("resources/space_invaders/invader_init_4"),
        (&screen::ENEMY_COLOR).into(),
        1
    )
    .unwrap()
    .to_fixed()
    .unwrap();
    static ref INVADER_INIT_5: FixedSpriteData = load_sprite_default(
        include_str!("resources/space_invaders/invader_init_5"),
        (&screen::ENEMY_COLOR).into(),
        1
    )
    .unwrap()
    .to_fixed()
    .unwrap();
    static ref INVADER_INIT_6: FixedSpriteData = load_sprite_default(
        include_str!("resources/space_invaders/invader_init_6"),
        (&screen::ENEMY_COLOR).into(),
        1
    )
    .unwrap()
    .to_fixed()
    .unwrap();
    static ref INVADER_FLIP_1: FixedSpriteData = load_sprite_default(
        include_str!("resources/space_invaders/invader_flip_1"),
        (&screen::ENEMY_COLOR).into(),
        1
    )
    .unwrap()
    .to_fixed()
    .unwrap();
    static ref INVADER_FLIP_2: FixedSpriteData = load_sprite_default(
        include_str!("resources/space_invaders/invader_flip_2"),
        (&screen::ENEMY_COLOR).into(),
        1
    )
    .unwrap()
    .to_fixed()
    .unwrap();
    static ref INVADER_FLIP_3: FixedSpriteData = load_sprite_default(
        include_str!("resources/space_invaders/invader_flip_3"),
        (&screen::ENEMY_COLOR).into(),
        1
    )
    .unwrap()
    .to_fixed()
    .unwrap();
    static ref INVADER_FLIP_4: FixedSpriteData = load_sprite_default(
        include_str!("resources/space_invaders/invader_flip_4"),
        (&screen::ENEMY_COLOR).into(),
        1
    )
    .unwrap()
    .to_fixed()
    .unwrap();
    static ref INVADER_FLIP_5: FixedSpriteData = load_sprite_default(
        include_str!("resources/space_invaders/invader_flip_5"),
        (&screen::ENEMY_COLOR).into(),
        1
    )
    .unwrap()
    .to_fixed()
    .unwrap();
    static ref INVADER_FLIP_6: FixedSpriteData = load_sprite_default(
        include_str!("resources/space_invaders/invader_flip_6"),
        (&screen::ENEMY_COLOR).into(),
        1
    )
    .unwrap()
    .to_fixed()
    .unwrap();
    static ref INVADER_HIT_1: FixedSpriteData = load_sprite_default(
        include_str!("resources/space_invaders/invader_hit_1"),
        (&screen::ENEMY_COLOR).into(),
        1
    )
    .unwrap()
    .to_fixed()
    .unwrap();
    static ref INVADER_HIT_2: FixedSpriteData = load_sprite_default(
        include_str!("resources/space_invaders/invader_hit_2"),
        (&screen::ENEMY_COLOR).into(),
        1
    )
    .unwrap()
    .to_fixed()
    .unwrap();
    static ref INVADER_HIT_3: FixedSpriteData = load_sprite_default(
        include_str!("resources/space_invaders/invader_hit_3"),
        (&screen::ENEMY_COLOR).into(),
        1
    )
    .unwrap()
    .to_fixed()
    .unwrap();
    static ref INVADER_HIT_4: FixedSpriteData = load_sprite_default(
        include_str!("resources/space_invaders/invader_hit_4"),
        (&screen::ENEMY_COLOR).into(),
        1
    )
    .unwrap()
    .to_fixed()
    .unwrap();
    static ref PLAYER_SPRITE: FixedSpriteData = load_sprite_default(
        include_str!("resources/player_ship"),
        (&screen::SHIP_COLOR).into(),
        1
    )
    .unwrap()
    .to_fixed()
    .unwrap();
}

#[derive(Clone, Serialize, Deserialize)]
pub enum Orientation {
    INIT,
    FLIP,
}

pub fn load_sprite(
    data: &str,
    on_color: Color,
    on_symbol: char,
    off_symbol: char,
    _scale: i32,
) -> Result<SpriteData, String> {
    let off_color = Color::invisible();
    let mut pixels = Vec::new();
    for line in data.lines() {
        let mut pixel_row = Vec::new();
        for ch in line.chars() {
            if ch == on_symbol {
                pixel_row.push(on_color);
            } else if ch == off_symbol {
                pixel_row.push(off_color);
            } else {
                return Err(format!(
                    "Cannot construct pixel from {}, expected one of (on={}, off={})",
                    ch, on_symbol, off_symbol
                ));
            }
        }
        pixels.push(pixel_row);
    }
    let width = pixels[0].len();
    debug_assert!(pixels.iter().all(|row| row.len() == width));
    Ok(SpriteData::new(pixels, 1))
}
pub fn load_sprite_default(data: &str, on_color: Color, scale: i32) -> Result<SpriteData, String> {
    load_sprite(data, on_color, 'X', '.', scale)
}

pub fn get_invader_sprite(enemy: &Enemy) -> FixedSpriteData {
    let row = enemy.row;
    let orientation = &enemy.orientation;
    if let Some(dc) = enemy.death_counter {
        for i in 0..4 {
            let bound = screen::DEATH_TIME - (screen::DEATH_HIT_1 + i * screen::DEATH_HIT_N);
            if dc > bound {
                match i + 1 {
                    1 => return INVADER_HIT_1.clone(),
                    2 => return INVADER_HIT_2.clone(),
                    3 => return INVADER_HIT_3.clone(),
                    4 => return INVADER_HIT_4.clone(),
                    _ => unreachable!("There are only 4 animations."),
                }
            }
        }
        unreachable!("Something is messed up in your death clock.")
    } else {
        match (row + 1, orientation) {
            (1, Orientation::INIT) => INVADER_INIT_1.clone(),
            (2, Orientation::INIT) => INVADER_INIT_2.clone(),
            (3, Orientation::INIT) => INVADER_INIT_3.clone(),
            (4, Orientation::INIT) => INVADER_INIT_4.clone(),
            (5, Orientation::INIT) => INVADER_INIT_5.clone(),
            (6, Orientation::INIT) => INVADER_INIT_6.clone(),
            (1, Orientation::FLIP) => INVADER_FLIP_1.clone(),
            (2, Orientation::FLIP) => INVADER_FLIP_2.clone(),
            (3, Orientation::FLIP) => INVADER_FLIP_3.clone(),
            (4, Orientation::FLIP) => INVADER_FLIP_4.clone(),
            (5, Orientation::FLIP) => INVADER_FLIP_5.clone(),
            (6, Orientation::FLIP) => INVADER_FLIP_6.clone(),
            _ => unreachable!("Only expecting 6 invader types"),
        }
    }
}

lazy_static! {
    static ref SHIELD_SPRITE: SpriteData = load_sprite_default(
        screen::SHIELD_SPRITE_DATA,
        (&screen::SHIELD_COLOR).into(),
        1
    )
    .expect("Shield sprite should be included!");
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Player {
    pub x: i32,
    pub y: i32,
    pub w: i32,
    pub h: i32,
    /// Speed of movenet.
    pub speed: i32,
    pub color: Color,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Laser {
    pub x: i32,
    pub y: i32,
    pub w: i32,
    pub h: i32,
    /// Laser timing (visible / not-visible) based on this.
    pub t: i32,
    /// Lasers have a direction.
    pub movement: Direction,
    pub speed: i32,
    pub color: Color,
}

impl Player {
    fn new(x: i32, y: i32) -> Player {
        let (w, h) = screen::SHIP_SIZE;
        Player {
            x,
            y,
            w,
            h,
            speed: 3,
            color: (&screen::SHIP_COLOR).into(),
        }
    }
    fn _rect(&self) -> Rect {
        Rect::new(self.x, self.y, self.w, self.h)
    }
}

impl Laser {
    fn new(x: i32, y: i32, dir: Direction) -> Laser {
        let w = screen::LASER_SIZE_W;
        let h = screen::LASER_SIZE_H1;
        Laser {
            x,
            y,
            w,
            h,
            t: 0,
            color: (&screen::LASER_COLOR).into(),
            movement: dir,
            speed: 3,
        }
    }
    /// Every other frame a laser is visible.
    fn is_visible(&self) -> bool {
        self.t % 4 < 2
    }
    fn rect(&self) -> Rect {
        Rect::new(self.x, self.y, self.w, self.h)
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct State {
    pub rand: random::Gen,
    pub life_display_timer: i32,
    pub lives: i32,
    pub score: i32,
    /// Ship is a rectangular actor (logically).
    pub ship: Player,
    /// Emulate the fact that Atari could only have one laser at a time (and it "recharges" faster if you hit the front row...)
    pub ship_laser: Option<Laser>,
    /// Shields are destructible, so we need to track their pixels...
    pub shields: Vec<SpriteData>,
    /// Enemies are rectangular actors (logically speaking).
    pub enemies: Vec<Enemy>,
    /// Enemy shot delay:
    pub enemy_shot_delay: i32,
    /// Enemy lasers are actors as well.
    pub enemy_lasers: Vec<Laser>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Enemy {
    x: i32,
    y: i32,
    row: i32,
    col: i32,
    id: u32,
    alive: bool,
    death_counter: Option<i32>,
    move_counter: i32,
    move_right: bool,
    move_down: bool,
    orientation: Orientation,
}

impl Enemy {
    fn new(x: i32, y: i32, row: i32, col: i32, id: u32) -> Enemy {
        Enemy {
            x,
            y,
            row,
            col,
            id,
            alive: true,
            death_counter: None,
            move_counter: screen::ENEMY_PERIOD,
            move_right: true,
            move_down: true,
            orientation: Orientation::INIT,
        }
    }
    fn rect(&self) -> Rect {
        Rect::new(self.x, self.y, screen::ENEMY_SIZE.0, screen::ENEMY_SIZE.1)
    }
    fn enemy_shift(&mut self) {
        if self.move_counter == 0 {
            let (width, height) = screen::ENEMY_SIZE;
            let (deltax, deltay) = screen::ENEMY_DELTA;
            let (padx, pady) = screen::ENEMY_SPACE;
            let startx = screen::ENEMY_START_POS.0 + self.col * (width + padx);
            let starty = screen::ENEMY_START_POS.1 + self.row * (height + pady);
            let end = screen::ENEMY_END_POS.0 + self.col * (width + padx);
            // If this is the first move, just move right.
            // move_right flag is initialized to be true, so we don't need to change it.
            if self.x == startx && self.y == starty {
                self.x += deltax;
            // If we are aligned with the start position on the x-axis, set movement
            // direction to the right, move down, and toggle the move down flag
            } else if self.x == startx && self.move_down {
                self.y += deltay;
                self.move_right = true;
                self.move_down = false;
            // Likewise for the end position
            } else if self.x == end && self.move_down {
                self.y += deltay;
                self.move_right = false;
                self.move_down = false;
            // If we aren't at the (x,y) start position and aren't moving down, move laterally.
            } else if self.move_right {
                self.x += deltax;
                self.move_down = true;
            } else {
                self.x -= deltax;
                self.move_down = true;
            };
            self.orientation = match self.orientation {
                Orientation::INIT => Orientation::FLIP,
                Orientation::FLIP => Orientation::INIT,
            };
            self.move_counter = screen::ENEMY_PERIOD;
        } else {
            self.move_counter -= 1;
        }
    }
}

impl State {
    fn new(rand: random::Gen) -> State {
        let player_start_x = screen::SHIP_LIMIT_X1;
        let player_start_y = screen::SKY_TO_GROUND - screen::SHIP_SIZE.1;
        let mut shields = Vec::new();
        let mut enemies = Vec::new();

        for &(x, y) in &[
            screen::SHIELD1_POS,
            screen::SHIELD2_POS,
            screen::SHIELD3_POS,
        ] {
            shields.push(SHIELD_SPRITE.translate(x, y))
        }

        let (x, y) = screen::ENEMY_START_POS;
        let (w, h) = screen::ENEMY_SIZE;
        let x_offset = w + screen::ENEMY_SPACE.0;
        let y_offset = h + screen::ENEMY_SPACE.1;
        for j in 0..screen::ENEMIES_NUM {
            for i in 0..screen::ENEMIES_PER_ROW {
                let x = x + (i * x_offset);
                let y = y + (j * y_offset);
                let id = enemies.len() as u32;
                enemies.push(Enemy::new(x, y, j, i, id));
            }
        }

        State {
            rand,
            life_display_timer: screen::NEW_LIFE_TIME,
            lives: 3,
            score: 0,
            ship: Player::new(player_start_x, player_start_y),
            ship_laser: None,
            enemy_shot_delay: 50,
            shields,
            enemies,
            enemy_lasers: Vec::new(),
        }
    }

    /// Find the enemy, if any, hit by the ship_laser, and start its death timer.
    fn laser_enemy_collisions(&mut self) {
        let mut hit = None;
        if let Some(laser) = &mut self.ship_laser {
            let laser_rect = laser.rect();

            // Check collision with living enemies:
            for e in self.enemies.iter().filter(|e| e.alive) {
                let enemy_rect = e.rect();

                // Broad-phase collision: is it in the rectangle?
                if laser_rect.intersects(&enemy_rect) {
                    let sprite = get_invader_sprite(&e);
                    // pixel-perfect detection:
                    if laser_rect.collides_visible(e.x, e.y, &sprite.data) {
                        hit = Some(e.id);
                        break;
                    }
                }
            }
        }

        // Start enemy death animations.
        if let Some(eid) = hit {
            let enemy = &mut self.enemies[eid as usize];
            self.ship_laser = None;
            if enemy.death_counter.is_none() {
                enemy.death_counter = Some(screen::DEATH_TIME)
            }
        }
    }

    /// Run through any enemy death animation timers, marking them as dead when finished!
    fn enemy_animation(&mut self) {
        for enemy in self.enemies.iter_mut() {
            if let Some(dc) = enemy.death_counter {
                let dc = dc - 1;
                if dc == 0 {
                    enemy.death_counter = None;
                    enemy.alive = false;
                    self.score += 10;
                } else {
                    enemy.death_counter = Some(dc);
                }
            }
        }
    }

    /// Deletes the laser if it has gone off the top of the screen.
    fn laser_miss_check(&mut self) {
        if self
            .ship_laser
            .as_ref()
            .map(|laser| laser.y < 0)
            .unwrap_or(false)
        {
            self.ship_laser = None;
        }

        // Collect lasers that will have gone off-screen:
        let mut delete = Vec::new();
        for (idx, laser) in self.enemy_lasers.iter().enumerate() {
            if laser.y > screen::GAME_SIZE.1 {
                delete.push(idx);
            }
        }
        // Delete indexes in backwards order.
        for index in delete.into_iter().rev() {
            self.enemy_lasers.remove(index);
        }
    }

    fn laser_shield_check(&mut self, laser: &Rect) -> bool {
        // Check collision with living shields:
        for shield in self.shields.iter_mut() {
            let shield_rect = Rect::new(shield.x, shield.y, shield.width(), shield.height());

            // Broad-phase collision: is it in the rectangle?
            if laser.intersects(&shield_rect) {
                if destruction::destructive_collide(&laser, shield.x, shield.y, &mut shield.data) {
                    return true;
                }
            }
        }
        false
    }

    /// Move enemies
    fn enemy_shift(&mut self) {
        for enemy in self.enemies.iter_mut() {
            enemy.enemy_shift();
        }
    }

    fn enemy_fire_lasers(&mut self) {
        // Don't fire too many lasers.
        if self.enemy_lasers.len() > 1 {
            return;
        }
        self.enemy_shot_delay -= 1;
        if self.enemy_shot_delay <= 0 {
            // TODO: better delay? Less predictable?
            // Random state?
            self.enemy_shot_delay = 50;

            // Everybody shoots for now...
            for eid in self.active_weapon_enemy_ids() {
                let enemy = &mut self.enemies[eid as usize];
                let start = enemy.rect();

                let shot = Laser::new(start.center_x(), start.center_y(), Direction::Down);
                self.enemy_lasers.push(shot);
            }
        }
    }

    /// Find all enemies that are at the "bottom" of their column and can therefore fire weapons.
    /// Return ids in case you want to mutably or immutably borrow them.
    fn active_weapon_enemy_ids(&self) -> Vec<u32> {
        let mut out = Vec::new();

        let alive: Vec<&Enemy> = self.enemies.iter().filter(|e| e.alive).collect();
        let columns: Vec<i32> = alive.iter().map(|e| e.col).unique().collect();

        for col in columns.into_iter() {
            let bottom = alive
                .iter()
                .filter(|e| e.col == col)
                .max_by_key(|e| e.row)
                .map(|e| e.id);
            if let Some(eid) = bottom {
                out.push(eid);
            }
        }

        out
    }

    /// Move all lasers watching for collision with shields!
    fn enemy_laser_movement(&mut self) {
        let mut delete_lasers = Vec::new();
        for laser_idx in 0..self.enemy_lasers.len() {
            let laser_speed = self.enemy_lasers[laser_idx].speed;
            for _ in 0..laser_speed {
                self.enemy_lasers[laser_idx].y += 1;
                let laser_rectangle = self.enemy_lasers[laser_idx].rect();
                if self.laser_shield_check(&laser_rectangle) {
                    delete_lasers.push(laser_idx);
                    break;
                }
            }
        }
        for laser_idx in delete_lasers.into_iter().rev() {
            self.enemy_lasers.remove(laser_idx);
        }
    }
}

pub struct SpaceInvaders {
    pub rand: random::Gen,
}
impl Default for SpaceInvaders {
    fn default() -> Self {
        SpaceInvaders {
            rand: random::Gen::new_from_seed(17),
        }
    }
}
impl toybox_core::Simulation for SpaceInvaders {
    fn as_any(&self) -> &Any {
        self
    }
    fn reset_seed(&mut self, seed: u32) {
        self.rand.reset_seed(seed)
    }
    fn game_size(&self) -> (i32, i32) {
        screen::GAME_SIZE
    }
    fn new_game(&mut self) -> Box<toybox_core::State> {
        Box::new(State::new(random::Gen::new_child(&mut self.rand)))
    }
    /// Sync with [ALE impl](https://github.com/mgbellemare/Arcade-Learning-Environment/blob/master/src/games/supported/SpaceInvaders.cpp#L85)
    fn legal_action_set(&self) -> Vec<AleAction> {
        vec![
            AleAction::NOOP,
            AleAction::RIGHT,
            AleAction::RIGHTFIRE,
            AleAction::FIRE,
            AleAction::LEFT,
            AleAction::LEFTFIRE,
        ]
    }
    fn new_state_from_json(
        &self,
        json_str: &str,
    ) -> Result<Box<toybox_core::State>, serde_json::Error> {
        let state: State = serde_json::from_str(json_str)?;
        Ok(Box::new(state))
    }
    fn new_state_config_from_json(
        &self,
        _json_config: &str,
        _json_state: &str,
    ) -> Result<Box<toybox_core::State>, serde_json::Error> {
        panic!("No config implemented for SpaceInvaders.")
    }
}

impl toybox_core::State for State {
    fn as_any(&self) -> &Any {
        self
    }
    fn lives(&self) -> i32 {
        self.lives
    }
    fn score(&self) -> i32 {
        self.score
    }
    fn update_mut(&mut self, buttons: Input) {
        // Don't play game yet if displaying lives.
        if self.life_display_timer > 0 {
            self.life_display_timer -= 1;
            return;
        }

        if buttons.left {
            self.ship.x -= self.ship.speed;
        } else if buttons.right {
            self.ship.x += self.ship.speed;
        }

        if self.ship.x > screen::SHIP_LIMIT_X2 {
            self.ship.x = screen::SHIP_LIMIT_X2;
        } else if self.ship.x < screen::SHIP_LIMIT_X1 {
            self.ship.x = screen::SHIP_LIMIT_X1;
        }

        // Only shoot a laser if not present:
        if self.ship_laser.is_none() && buttons.button1 {
            self.ship_laser = Some(Laser::new(
                self.ship.x + self.ship.w / 2,
                self.ship.y,
                Direction::Up,
            ));
        }

        self.laser_enemy_collisions();
        self.enemy_shift();
        self.enemy_animation();
        self.enemy_fire_lasers();
        self.enemy_laser_movement();

        if self.ship_laser.is_some() {
            let laser_speed = self.ship_laser.as_ref().map(|l| l.speed).unwrap();

            // Move the laser 1px at a time (for collisions) within a frame up to its speed.
            for _ in 0..laser_speed {
                if let Some(laser) = &mut self.ship_laser {
                    laser.y -= 1;
                } else {
                    break;
                }
                if self.ship_laser.is_some() {
                    let laser = self.ship_laser.as_ref().map(|l| l.rect()).unwrap();
                    if self.laser_shield_check(&laser) {
                        self.ship_laser = None;
                        break;
                    }
                }
                self.laser_enemy_collisions();
            }
        }

        // See if lasers have gone off-screen.
        self.laser_miss_check();
    }

    fn draw(&self) -> Vec<Drawable> {
        let mut output = Vec::new();
        output.push(Drawable::rect(
            Color::black(),
            0,
            0,
            screen::GAME_SIZE.0,
            screen::GAME_SIZE.1,
        ));
        // draw ground:
        output.push(Drawable::rect(
            (&screen::GROUND_COLOR).into(),
            0,
            screen::SKY_TO_GROUND,
            screen::GAME_SIZE.0,
            screen::GAME_SIZE.1 - screen::SKY_TO_GROUND,
        ));
        // draw dots
        output.push(Drawable::rect(
            (&screen::LEFT_GAME_DOT_COLOR).into(),
            screen::GAME_DOT_LEFT,
            screen::SKY_TO_GROUND + 1,
            screen::GAME_DOT_SIZE.0,
            screen::GAME_DOT_SIZE.1,
        ));
        output.push(Drawable::rect(
            (&screen::RIGHT_GAME_DOT_COLOR).into(),
            screen::GAME_DOT_RIGHT,
            screen::SKY_TO_GROUND + 1,
            screen::GAME_DOT_SIZE.0,
            screen::GAME_DOT_SIZE.1,
        ));
        // draw score
        output.extend(draw_score(
            self.score,
            screen::SCORE_LEFT_X_POS,
            screen::SCORE_Y_POS,
            FontChoice::LEFT,
        ));
        output.extend(draw_score(
            0,
            screen::SCORE_RIGHT_X_POS,
            screen::SCORE_Y_POS,
            FontChoice::RIGHT,
        ));

        // Render lives font in the middle of the screen!
        if self.life_display_timer > 0 {
            output.push(Drawable::sprite(
                screen::LIVES_DISPLAY_POSITION.0,
                screen::LIVES_DISPLAY_POSITION.1,
                get_sprite(self.lives as u32, FontChoice::LIVES),
            ));
        }

        if self.lives() < 0 {
            return output;
        }

        output.push(Drawable::sprite(
            self.ship.x,
            self.ship.y,
            PLAYER_SPRITE.clone(),
        ));

        for shield in &self.shields {
            output.push(Drawable::DestructibleSprite(shield.clone()));
        }

        for enemy in self.enemies.iter().filter(|e| e.alive) {
            output.push(Drawable::sprite(
                enemy.x,
                enemy.y,
                get_invader_sprite(&enemy),
            ));
        }

        if let Some(ref laser) = self.ship_laser {
            if laser.is_visible() {
                output.push(Drawable::rect(
                    laser.color,
                    laser.x,
                    laser.y,
                    laser.w,
                    laser.h,
                ))
            }
        }

        for laser in &self.enemy_lasers {
            if laser.is_visible() {
                output.push(Drawable::rect(
                    laser.color,
                    laser.x,
                    laser.y,
                    laser.w,
                    laser.h,
                ));
            }
        }

        output
    }

    fn to_json(&self) -> String {
        serde_json::to_string(self).expect("Should be no JSON Serialization Errors.")
    }

    fn config_to_json(&self) -> String {
        panic!("No config implemented for SpaceInvaders.")
    }
}

#[cfg(test)]
mod tests {
    #[test]
    pub fn test_shield_sprite_size() {
        let sprite = super::SHIELD_SPRITE.clone();
        assert_eq!(super::screen::SHIELD_SIZE.0, sprite.width());
        assert_eq!(super::screen::SHIELD_SIZE.1, sprite.height());
    }

}
