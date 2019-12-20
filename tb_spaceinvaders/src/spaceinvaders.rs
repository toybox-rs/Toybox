use super::destruction;
use super::font::{draw_score, get_sprite, FontChoice};
use super::types::*;
use firing_ai::{enemy_fire_lasers, FiringAI};
use itertools::Itertools;
use serde_json;
use toybox_core::collision::Rect;
use toybox_core::graphics::{Color, Drawable, FixedSpriteData, SpriteData};
use toybox_core::random;
use toybox_core::{AleAction, Direction, Input, QueryError};

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
    #[cfg(test)]
    pub const SHIELD_SIZE: (i32, i32) = (16, 18);
    pub const SHIELD1_POS: (i32, i32) = (84, 157);
    pub const SHIELD2_POS: (i32, i32) = (148, 157);
    pub const SHIELD3_POS: (i32, i32) = (212, 157);

    pub const ENEMY_SIZE: (i32, i32) = (16, 10);
    pub const ENEMY_START_POS: (i32, i32) = (44, 31);
    pub const ENEMY_END_POS: (i32, i32) = (GAME_SIZE.0 - ENEMY_START_POS.0 - ENEMY_SPACE.0, 31);
    pub const ENEMIES_PER_ROW: i32 = 6;
    pub const ENEMIES_NUM: i32 = 6;
    pub const ENEMY_SPACE: (i32, i32) = (16, 8);
    pub const ENEMY_DELTA: (i32, i32) = (2, 10);
    pub const ENEMY_PERIOD: i32 = 32;

    pub const FLASH_PERIOD: i32 = 8;

    pub static ENEMY_SPEEDUPS: &'static [(i32, i32)] = &[(12, 16), (24, 8), (30, 4), (32, 2)];
    pub static ENEMY_POINTS: &'static [i32] = &[30, 30, 20, 20, 10, 10];

    pub const START_LIVES: i32 = 3;
    pub const NEW_LIFE_TIME: i32 = 128;
    pub const DEATH_TIME: i32 = 29;
    pub const DEATH_HIT_1: i32 = 5;
    pub const DEATH_HIT_N: i32 = 8;
    pub const PLAYER_DEATH_TIME: i32 = 115;

    // Mothership
    pub const UFO_SIZE: (i32, i32) = (14, 7);
    pub const UFO_PERIOD: i32 = 500;
    pub const UFO_START_POS: (i32, i32) = (-2, 12);
    pub const UFO_DELTA: i32 = 2;
    pub const UFO_BONUS: i32 = 100;

    pub const LASER_SIZE_W: i32 = 2;
    pub const LASER_SIZE_H1: i32 = 11;

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
        (&screen::ENEMY_COLOR).into()
    );
    static ref INVADER_INIT_2: FixedSpriteData = load_sprite_default(
        include_str!("resources/space_invaders/invader_init_2"),
        (&screen::ENEMY_COLOR).into()
    );
    static ref INVADER_INIT_3: FixedSpriteData = load_sprite_default(
        include_str!("resources/space_invaders/invader_init_3"),
        (&screen::ENEMY_COLOR).into()
    );
    static ref INVADER_INIT_4: FixedSpriteData = load_sprite_default(
        include_str!("resources/space_invaders/invader_init_4"),
        (&screen::ENEMY_COLOR).into()
    );
    static ref INVADER_INIT_5: FixedSpriteData = load_sprite_default(
        include_str!("resources/space_invaders/invader_init_5"),
        (&screen::ENEMY_COLOR).into()
    );
    static ref INVADER_INIT_6: FixedSpriteData = load_sprite_default(
        include_str!("resources/space_invaders/invader_init_6"),
        (&screen::ENEMY_COLOR).into()
    );
    static ref INVADER_FLIP_1: FixedSpriteData = load_sprite_default(
        include_str!("resources/space_invaders/invader_flip_1"),
        (&screen::ENEMY_COLOR).into()
    );
    static ref INVADER_FLIP_2: FixedSpriteData = load_sprite_default(
        include_str!("resources/space_invaders/invader_flip_2"),
        (&screen::ENEMY_COLOR).into()
    );
    static ref INVADER_FLIP_3: FixedSpriteData = load_sprite_default(
        include_str!("resources/space_invaders/invader_flip_3"),
        (&screen::ENEMY_COLOR).into()
    );
    static ref INVADER_FLIP_4: FixedSpriteData = load_sprite_default(
        include_str!("resources/space_invaders/invader_flip_4"),
        (&screen::ENEMY_COLOR).into()
    );
    static ref INVADER_FLIP_5: FixedSpriteData = load_sprite_default(
        include_str!("resources/space_invaders/invader_flip_5"),
        (&screen::ENEMY_COLOR).into()
    );
    static ref INVADER_FLIP_6: FixedSpriteData = load_sprite_default(
        include_str!("resources/space_invaders/invader_flip_6"),
        (&screen::ENEMY_COLOR).into()
    );
    static ref INVADER_HIT_1: FixedSpriteData = load_sprite_default(
        include_str!("resources/space_invaders/invader_hit_1"),
        (&screen::ENEMY_COLOR).into()
    );
    static ref INVADER_HIT_2: FixedSpriteData = load_sprite_default(
        include_str!("resources/space_invaders/invader_hit_2"),
        (&screen::ENEMY_COLOR).into()
    );
    static ref INVADER_HIT_3: FixedSpriteData = load_sprite_default(
        include_str!("resources/space_invaders/invader_hit_3"),
        (&screen::ENEMY_COLOR).into()
    );
    static ref INVADER_HIT_4: FixedSpriteData = load_sprite_default(
        include_str!("resources/space_invaders/invader_hit_4"),
        (&screen::ENEMY_COLOR).into()
    );
    static ref PLAYER_SPRITE: FixedSpriteData = load_sprite_default(
        include_str!("resources/player_ship"),
        (&screen::SHIP_COLOR).into()
    );
    static ref PLAYER_HIT_1: FixedSpriteData = load_sprite_default(
        include_str!("resources/player_ship_hit_1"),
        (&screen::SHIP_COLOR).into()
    );
    static ref PLAYER_HIT_2: FixedSpriteData = load_sprite_default(
        include_str!("resources/player_ship_hit_2"),
        (&screen::SHIP_COLOR).into()
    );
    static ref UFO_MOTHERSHIP_SPRITE: FixedSpriteData = load_sprite_default(
        include_str!("resources/space_invaders/mothership"),
        (&screen::UFO_COLOR).into()
    );
    static ref UFO_HIT_1: FixedSpriteData = load_sprite_default(
        include_str!("resources/space_invaders/invader_hit_1"),
        (&screen::UFO_COLOR).into()
    );
    static ref UFO_HIT_2: FixedSpriteData = load_sprite_default(
        include_str!("resources/space_invaders/invader_hit_2"),
        (&screen::UFO_COLOR).into()
    );
    static ref UFO_HIT_3: FixedSpriteData = load_sprite_default(
        include_str!("resources/space_invaders/invader_hit_3"),
        (&screen::UFO_COLOR).into()
    );
    static ref UFO_HIT_4: FixedSpriteData = load_sprite_default(
        include_str!("resources/space_invaders/invader_hit_4"),
        (&screen::UFO_COLOR).into()
    );
}

pub fn load_sprite(
    data: &str,
    on_color: Color,
    on_symbol: char,
    off_symbol: char,
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
    Ok(SpriteData::new(pixels))
}
pub fn load_sprite_dynamic(data: &str, on_color: Color) -> Result<SpriteData, String> {
    load_sprite(data, on_color, 'X', '.')
}
pub fn load_sprite_default(data: &str, on_color: Color) -> FixedSpriteData {
    load_sprite_dynamic(data, on_color).unwrap().to_fixed()
}

pub fn get_invader_sprite(enemy: &Enemy, orientation: bool) -> FixedSpriteData {
    let row = enemy.row;
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
            (1, true) => INVADER_INIT_1.clone(),
            (2, true) => INVADER_INIT_2.clone(),
            (3, true) => INVADER_INIT_3.clone(),
            (4, true) => INVADER_INIT_4.clone(),
            (5, true) => INVADER_INIT_5.clone(),
            (6, true) => INVADER_INIT_6.clone(),
            (1, false) => INVADER_FLIP_1.clone(),
            (2, false) => INVADER_FLIP_2.clone(),
            (3, false) => INVADER_FLIP_3.clone(),
            (4, false) => INVADER_FLIP_4.clone(),
            (5, false) => INVADER_FLIP_5.clone(),
            (6, false) => INVADER_FLIP_6.clone(),
            _ => unreachable!("Only expecting 6 invader types"),
        }
    }
}

fn get_player_sprite(ship: &Player, life_display_timer: i32) -> Option<FixedSpriteData> {
    if ship.alive {
        Some(PLAYER_SPRITE.clone())
    } else if ship.death_counter.is_none() {
        // Between lives. Flash time.
        if (life_display_timer / screen::FLASH_PERIOD) % 2 == 1 {
            Some(PLAYER_SPRITE.clone())
        } else {
            None
        }
    } else if ship.death_hit_1 {
        Some(PLAYER_HIT_1.clone())
    } else {
        Some(PLAYER_HIT_2.clone())
    }
}

fn get_ufo_sprite(ufo: &Ufo) -> Option<FixedSpriteData> {
    // For now, just assume the same period as the enemy sprites.
    if let Some(dc) = ufo.death_counter {
        for i in 0..4 {
            let bound = screen::DEATH_TIME - (screen::DEATH_HIT_1 + i * screen::DEATH_HIT_N);
            if dc > bound {
                match i + 1 {
                    1 => return Some(UFO_HIT_1.clone()),
                    2 => return Some(UFO_HIT_2.clone()),
                    3 => return Some(UFO_HIT_3.clone()),
                    4 => return Some(UFO_HIT_4.clone()),
                    _ => unreachable!("There are only 4 animations."),
                }
            }
        }
        None
    } else {
        Some(UFO_MOTHERSHIP_SPRITE.clone())
    }
}

lazy_static! {
    static ref SHIELD_SPRITE: SpriteData =
        load_sprite_dynamic(screen::SHIELD_SPRITE_DATA, (&screen::SHIELD_COLOR).into(),)
            .expect("Shield sprite should be included!");
}

impl Default for SpaceInvaders {
    fn default() -> Self {
        SpaceInvaders {
            rand: random::Gen::new_from_seed(17),
            row_scores: screen::ENEMY_POINTS.to_vec(),
            start_lives: screen::START_LIVES,
            enemy_protocol: FiringAI::TargetPlayer,
            jitter: 0.5,
            shields: vec![
                screen::SHIELD1_POS,
                screen::SHIELD2_POS,
                screen::SHIELD3_POS,
            ],
        }
    }
}

impl Default for EnemiesMovementState {
    fn default() -> Self {
        EnemiesMovementState {
            move_counter: screen::ENEMY_PERIOD,
            move_dir: Direction::Right,
            visual_orientation: true,
        }
    }
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
            alive: false,
            death_counter: None,
            death_hit_1: true,
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

impl Ufo {
    fn new() -> Ufo {
        Ufo {
            x: screen::UFO_START_POS.0,
            y: screen::UFO_START_POS.1,
            appearance_counter: Some(screen::UFO_PERIOD),
            death_counter: None,
        }
    }
    fn rect(&self) -> Rect {
        Rect::new(self.x, self.y, screen::UFO_SIZE.0, screen::UFO_SIZE.1)
    }
    fn start_death_counter(&mut self) {
        self.death_counter = Some(screen::DEATH_TIME);
    }
    fn shift_ship(&mut self) {
        if self.x >= screen::GAME_SIZE.0 - screen::UFO_SIZE.0 {
            self.reset_mothership();
        } else {
            self.x += screen::UFO_DELTA;
        }
    }
    fn reset_mothership(&mut self) {
        self.x = screen::UFO_START_POS.0;
        self.y = screen::UFO_START_POS.1;
        self.appearance_counter = Some(screen::UFO_PERIOD);
        self.death_counter = None;
    }
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
            points: screen::ENEMY_POINTS[row as usize],
            death_counter: None,
        }
    }
    fn rect(&self) -> Rect {
        Rect::new(self.x, self.y, screen::ENEMY_SIZE.0, screen::ENEMY_SIZE.1)
    }
}

impl StateCore {
    /// Constructor based on configuration object.
    fn new(config: &mut SpaceInvaders) -> StateCore {
        let player_start_x = screen::SHIP_LIMIT_X1;
        let player_start_y = screen::SKY_TO_GROUND - screen::SHIP_SIZE.1;
        let mut state = StateCore {
            rand: random::Gen::new_child(&mut config.rand),
            life_display_timer: screen::NEW_LIFE_TIME,
            lives: 3,
            levels_completed: 0,
            score: 0,
            ship: Player::new(player_start_x, player_start_y),
            ship_laser: None,
            enemy_shot_delay: 50,
            shields: Vec::new(),
            enemies: Vec::new(),
            enemies_movement: Default::default(),
            enemy_lasers: Vec::new(),
            ufo: Ufo::new(),
        };

        state.reset_board(config);
        state
    }

    fn reset_board(&mut self, config: &SpaceInvaders) {
        self.shields.clear();
        self.enemies.clear();
        self.life_display_timer = screen::NEW_LIFE_TIME;
        self.ship.x = screen::SHIP_LIMIT_X1;
        self.enemy_lasers.clear();
        self.ship_laser = None;
        self.ufo = Ufo::new();

        for &(x, y) in config.shields.iter() {
            self.shields.push(SHIELD_SPRITE.translate(x, y))
        }

        let (x, mut y) = screen::ENEMY_START_POS;
        // start position should get lower with each level
        y += self.levels_completed * 2;
        let (w, h) = screen::ENEMY_SIZE;
        let x_offset = w + screen::ENEMY_SPACE.0;
        let y_offset = h + screen::ENEMY_SPACE.1;
        for j in 0..screen::ENEMIES_NUM {
            for i in 0..screen::ENEMIES_PER_ROW {
                let x = x + (i * x_offset);
                let y = y + (j * y_offset);
                let id = self.enemies.len() as u32;
                self.enemies.push(Enemy::new(x, y, j, i, id));
            }
        }
    }

    /// Flash the player ship and display lives
    fn flash_display_lives(&mut self) {
        if self.life_display_timer > 0 {
            self.life_display_timer -= 1;
        } else {
            self.ship.alive = true;
            self.life_display_timer = screen::NEW_LIFE_TIME;
        }
    }

    /// Find the laser, if any, that hits the ship, and start the ship's death timer.
    fn laser_player_collision(&mut self) {
        let x = self.ship.x;
        let y = self.ship.y;
        for laser in &self.enemy_lasers {
            let laser_rect = laser.rect();
            let player_rect = Rect::new(x, y, self.ship.w, self.ship.h);
            let player_sprite = PLAYER_SPRITE.clone();
            if laser_rect.intersects(&player_rect) {
                if laser_rect.collides_visible(x, y, &player_sprite.data) {
                    self.ship.alive = false;
                    self.ship.death_counter = Some(screen::PLAYER_DEATH_TIME);
                }
            }
        }
    }

    /// Decrement the player's death counter and update the sprite image
    fn player_toggle_death(&mut self) {
        if let Some(counter) = self.ship.death_counter {
            if counter > 0 {
                let counter = counter - 1;
                if counter % 24 == 0 {
                    self.ship.death_hit_1 = false;
                } else if counter % 12 == 0 {
                    self.ship.death_hit_1 = true;
                }
                self.ship.death_counter = Some(counter);
            } else {
                self.ship.death_counter = None;
                self.lives -= 1;
                // self.ship.alive = true;
                self.ship.x = screen::SHIP_LIMIT_X1;
            }
        }
    }

    /// Find the enemy, if any, hit by the ship_laser, and start its death timer.
    fn laser_enemy_collisions(&mut self) {
        let mut hit = None;
        if let Some(laser) = &mut self.ship_laser {
            let laser_rect = laser.rect();

            let enemy_orient = self.enemies_movement.visual_orientation;

            // Check collision with living enemies:
            for e in self
                .enemies
                .iter()
                .filter(|e| e.alive && e.death_counter.is_none())
            {
                let enemy_rect = e.rect();

                // Broad-phase collision: is it in the rectangle?
                if laser_rect.intersects(&enemy_rect) {
                    let sprite = get_invader_sprite(&e, enemy_orient);
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

    // If the player's laser has hit the UFO, start its death timer.
    fn laser_ufo_collision(&mut self) {
        // If the UFO has not yet appeared, it can't be hit.
        if self.ufo.appearance_counter.is_some() {
            return;
        }
        // If the UFO is currently dying, it can't be hit.
        if self.ufo.death_counter.is_some() {
            return;
        }

        let ufo = &mut self.ufo;
        if let Some(ref mut laser) = self.ship_laser {
            let laser_rect = laser.rect();
            let ufo_rect = ufo.rect();
            if laser_rect.intersects(&ufo_rect) {
                if let Some(sprite) = get_ufo_sprite(&ufo) {
                    if laser_rect.collides_visible(ufo.x, ufo.y, &sprite.data) {
                        ufo.start_death_counter();
                        self.score += screen::UFO_BONUS;
                    }
                } else {
                    unreachable!("We should have exited earlier if the death counter is 0 and sprite is None.");
                }
            }
        }
        //
        if ufo.death_counter.is_some() {
            self.ship_laser = None;
        }
    }

    /// Handle counters for ufo appearance, collision, etc.    
    fn laser_ufo_movement_animation(&mut self) {
        if let Some(counter) = self.ufo.appearance_counter {
            // UFO has not already appeared
            self.ufo.appearance_counter = if counter == 0 {
                None
            } else {
                Some(counter - 1)
            }
        } else if let Some(counter) = self.ufo.death_counter {
            // UFO has appeared and has been hit.
            if counter == 0 {
                self.ufo = Ufo::new();
            } else {
                self.ufo.death_counter = Some(counter - 1);
            }
        } else {
            self.ufo.shift_ship();
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
                    self.score += enemy.points;
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

    /// Move enemies as a group!
    fn enemy_shift(&mut self) {
        // Enemies do not move every frame; there is a delay.
        if self.enemies_movement.move_counter > 0 {
            self.enemies_movement.move_counter -= 1;
            return;
        }

        // Calculate the rectangle that surrounds all living enemies.
        let rectangle: Vec<Rect> = self
            .enemies
            .iter()
            .filter(|e| e.alive)
            .map(|e| e.rect())
            .collect();
        let rect_union = Rect::merge(&rectangle);
        if rect_union == None {
            return;
        }
        let rect_union = rect_union.unwrap();
        // Calculate how many deaths have occurred:
        let num_killed = (self.enemies.len() - rectangle.len()) as i32;

        // Calculate which direction we are moving, and what next to do.
        let (deltax, deltay) = screen::ENEMY_DELTA;
        let startx = screen::ENEMY_START_POS.0;
        let end = screen::ENEMY_END_POS.0;

        let x = rect_union.x1();

        // we're going to fill in the change to apply to all enemies.
        let mut dx = 0;
        let mut dy = 0;

        // Grab a reference to the state variables we need.
        let mut movement = &mut self.enemies_movement;

        // They're moving, so flip the sprites.
        movement.visual_orientation = !movement.visual_orientation;

        let at_left = x <= startx;
        let at_right = rect_union.x2() >= end;

        let new_dir = match movement.move_dir {
            Direction::Left => {
                if at_left {
                    Direction::Down
                } else {
                    dx = -deltax;
                    Direction::Left
                }
            }
            Direction::Right => {
                if at_right {
                    Direction::Down
                } else {
                    dx = deltax;
                    Direction::Right
                }
            }
            Direction::Down => {
                dy = deltay;
                if at_left {
                    Direction::Right
                } else {
                    Direction::Left
                }
            }
            Direction::Up => unreachable!(),
        };

        // Change direction if necessary.
        movement.move_dir = new_dir;

        // Accelerate appropriately.
        for (num_dead, speedup) in screen::ENEMY_SPEEDUPS {
            if num_killed >= *num_dead {
                movement.move_counter = *speedup;
            }
        }
        // If we haven't killed enough enemies to accelerate, reset to the original period.
        if movement.move_counter == 0 {
            movement.move_counter = screen::ENEMY_PERIOD;
        }

        // Now actually move enemies!
        for e in self.enemies.iter_mut() {
            e.x += dx;
            // Make sure we don't go off the edge!
            //e.x = max(startx, min(e.x, end));
            e.y += dy;
        }
    }

    /// Determine the closest enemy for firing patterns; returns an index to avoid the borrow-checker.
    pub fn closest_enemy_id(&self) -> u32 {
        let playerx = &self.ship.x;
        let mut dist = screen::GAME_SIZE.0;
        let mut closest_id = 0;
        for eid in self.active_weapon_enemy_ids() {
            let e = &self.enemies[eid as usize];
            let edist = (e.x - playerx).abs();
            if edist < dist {
                dist = edist;
                closest_id = eid;
            }
        }
        closest_id
    }

    fn enemy_fire_lasers(&mut self, config: &SpaceInvaders) {
        // Don't fire too many lasers.
        if self.enemy_lasers.len() > 1 {
            return;
        }
        self.enemy_shot_delay -= 1;
        if self.enemy_shot_delay <= 0 {
            self.enemy_shot_delay = 50;
            let shooter_index = enemy_fire_lasers(self, config);
            let shooter = &self.enemies[shooter_index as usize];
            let start = shooter.rect();
            let shot = Laser::new(start.center_x(), start.center_y(), Direction::Down);
            self.enemy_lasers.push(shot);
        }
    }

    /// Find all enemies that are at the "bottom" of their column and can therefore fire weapons.
    /// Return ids in case you want to mutably or immutably borrow them.
    pub fn active_weapon_enemy_ids(&self) -> Vec<u32> {
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

    /// If enemies have moved far enough down to overlap with shields,
    /// then shields are removed.
    fn remove_shields(&mut self) {
        for id in self.active_weapon_enemy_ids() {
            let enemy = &self.enemies[id as usize];
            // We only care about the lowest enemies. However, since
            // we have to loop through all of the enemies to find the lowest
            // one anyway, we might as well just check here.
            let lower_bound = enemy.y + screen::ENEMY_SIZE.1;
            // Not sure if shields disappear at the bounding box intersection,
            // or at actual pixel overlap (put another way: do partially
            // destructed shields lead to a delay in their final disappearance?)
            // Going with the simpler version for now.
            if lower_bound > screen::SHIELD1_POS.1 {
                self.shields = Vec::new();
            }
        }
    }
    fn has_lost(&self) -> bool {
        self.enemies
            .iter()
            .any(|e| e.alive && e.rect().y2() >= screen::SKY_TO_GROUND)
    }
    fn has_won(&self) -> bool {
        self.enemies.iter().all(|e| !e.alive)
    }
    fn reset_condition(&self) -> bool {
        let game_over = self.lives < 0;
        let won = self.has_won();
        let lost = self.has_lost();
        game_over || won || lost
    }
}

impl toybox_core::Simulation for SpaceInvaders {
    fn reset_seed(&mut self, seed: u32) {
        self.rand.reset_seed(seed)
    }
    fn game_size(&self) -> (i32, i32) {
        screen::GAME_SIZE
    }
    fn new_game(&mut self) -> Box<dyn toybox_core::State> {
        Box::new(State {
            config: self.clone(),
            state: StateCore::new(self),
        })
    }
    /// Sync with [ALE impl](https://github.com/mgbellemare/Arcade-Learning-Environment/blob/master/src/games/supported/SpaceInvaders.cpp#L85)
    /// Note, leaving a call to sort in this impl to remind users that these vecs are ordered!
    fn legal_action_set(&self) -> Vec<AleAction> {
        let mut actions = vec![
            AleAction::NOOP,
            AleAction::RIGHT,
            AleAction::RIGHTFIRE,
            AleAction::FIRE,
            AleAction::LEFT,
            AleAction::LEFTFIRE,
        ];
        actions.sort();
        actions
    }
    fn new_state_from_json(
        &self,
        json_str: &str,
    ) -> Result<Box<dyn toybox_core::State>, serde_json::Error> {
        let state: StateCore = serde_json::from_str(json_str)?;
        Ok(Box::new(State {
            state,
            config: self.clone(),
        }))
    }

    fn to_json(&self) -> String {
        serde_json::to_string(self).expect("SpaceInvaders should be JSON-serializable!")
    }

    fn from_json(
        &self,
        json_str: &str,
    ) -> Result<Box<dyn toybox_core::Simulation>, serde_json::Error> {
        let config: SpaceInvaders = serde_json::from_str(json_str)?;
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
        if self.state.reset_condition() {
            // If enemies hit the earth, you have lost. Game is over.
            if self.state.has_lost() {
                self.state.lives = -1;
                return;
            } else if self.state.has_won() {
                self.state.levels_completed += 1;
            }

            self.state.reset_board(&self.config);
            return;
        }
        if self.state.ship.alive {
            // The ship can only move if it is alive.
            if buttons.left {
                self.state.ship.x -= self.state.ship.speed;
            } else if buttons.right {
                self.state.ship.x += self.state.ship.speed;
            }

            if self.state.ship.x > screen::SHIP_LIMIT_X2 {
                self.state.ship.x = screen::SHIP_LIMIT_X2;
            } else if self.state.ship.x < screen::SHIP_LIMIT_X1 {
                self.state.ship.x = screen::SHIP_LIMIT_X1;
            }

            // Only shoot a laser if not present and if we aren't in the throes of death:
            if self.state.ship_laser.is_none() && buttons.button1 {
                self.state.ship_laser = Some(Laser::new(
                    self.state.ship.x + self.state.ship.w / 2,
                    self.state.ship.y,
                    Direction::Up,
                ));
            }
            // Enemies only move if the player is alive
            self.state.enemy_shift();
            // Enemies only fire if the player is alive
            self.state.enemy_fire_lasers(&self.config);
            self.state.remove_shields();

            // See if lasers have gone off-screen.
            self.state.laser_miss_check();

            // Only check and update the UFO if the player is alive
            self.state.laser_ufo_movement_animation();
            self.state.laser_ufo_collision();
        } else if self.state.ship.death_counter.is_none() {
            self.state.flash_display_lives();
            self.state.ufo.reset_mothership();
        }

        // Player's laser continues moving during death.
        if self.state.ship_laser.is_some() {
            let laser_speed = self.state.ship_laser.as_ref().map(|l| l.speed).unwrap();

            // Move the laser 1px at a time (for collisions) within a frame up to its speed.
            for _ in 0..laser_speed {
                if let Some(laser) = &mut self.state.ship_laser {
                    laser.y -= 1;
                } else {
                    break;
                }
                if self.state.ship_laser.is_some() {
                    let laser = self.state.ship_laser.as_ref().map(|l| l.rect()).unwrap();
                    if self.state.laser_shield_check(&laser) {
                        self.state.ship_laser = None;
                        break;
                    }
                }
                self.state.laser_enemy_collisions();
            }
        }
        self.state.enemy_animation();
        self.state.enemy_laser_movement();
        self.state.laser_player_collision();
        self.state.player_toggle_death();
    }

    fn draw(&self) -> Vec<Drawable> {
        let mut output = Vec::new();
        output.push(Drawable::Clear(Color::black()));
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
        // draw score or mothership
        if self.state.ufo.appearance_counter.is_none() {
            if let Some(ufo_sprite) = get_ufo_sprite(&self.state.ufo) {
                output.push(Drawable::sprite(
                    self.state.ufo.x,
                    self.state.ufo.y,
                    ufo_sprite,
                ));
            }
        } else {
            output.extend(draw_score(
                self.state.score % 10000,
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
        }

        if self.lives() < 0 {
            return output;
        }

        if let Some(player_sprite) =
            get_player_sprite(&self.state.ship, self.state.life_display_timer)
        {
            output.push(Drawable::sprite(
                self.state.ship.x,
                self.state.ship.y,
                player_sprite.clone(),
            ));
        }

        // In between lives.
        if !self.state.ship.alive && self.state.ship.death_counter.is_none() {
            output.push(Drawable::sprite(
                screen::LIVES_DISPLAY_POSITION.0,
                screen::LIVES_DISPLAY_POSITION.1,
                get_sprite(self.state.lives as u32, FontChoice::LIVES),
            ));
        }

        for shield in &self.state.shields {
            output.push(Drawable::DestructibleSprite(shield.clone()));
        }

        // Calculate the rectangle that surrounds all living enemies.
        let rectangle: Vec<Rect> = self
            .state
            .enemies
            .iter()
            .filter(|e| e.alive)
            .map(|e| e.rect())
            .collect();
        let rect_union = Rect::merge(&rectangle);
        if let Some(rect) = rect_union {
            output.push(Drawable::rect(
                Color::rgb(100, 0, 0),
                rect.x,
                rect.y,
                rect.w,
                rect.h,
            ));
        }

        let enemy_orient = self.state.enemies_movement.visual_orientation;
        for enemy in self.state.enemies.iter().filter(|e| e.alive) {
            output.push(Drawable::sprite(
                enemy.x,
                enemy.y,
                get_invader_sprite(&enemy, enemy_orient),
            ));
        }

        if let Some(ref laser) = self.state.ship_laser {
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

        for laser in &self.state.enemy_lasers {
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
        serde_json::to_string(&self.state).expect("Should be no JSON Serialization Errors.")
    }

    fn query_json(&self, query: &str, _args: &serde_json::Value) -> Result<String, QueryError> {
        let _config = &self.config;
        let state = &self.state;
        Ok(match query {
            "ship_xy" => serde_json::to_string(&(state.ship.x, state.ship.y))?,
            "ship_x" => serde_json::to_string(&state.ship.x)?,
            "shield_count" => serde_json::to_string(&state.shields.len())?,
            "shields" => serde_json::to_string(&state.shields)?,
            _ => Err(QueryError::NoSuchQuery)?,
        })
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
