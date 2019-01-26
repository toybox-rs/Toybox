// Contains predicates that are useful for interventions.
// These can all be implemented over json, but exporting some fast
// ones from Rust will be helpful.

#[cfg(feature = "amidar")]
pub mod amidar {
    use super::super::amidar::State;
    pub fn num_tiles_unpainted(state: &State) -> i32 {
        let mut sum = 0;
        for row in state.state.board.tiles.iter() {
            sum += row
                .iter()
                .filter(|t| t.walkable())
                .filter(|t| t.needs_paint())
                .count();
        }
        sum as i32
    }

    pub fn regular_mode(state: &State) -> bool {
        state.state.chase_timer == 0 && state.state.jump_timer == 0
    }

    pub fn chase_mode(state: &State) -> bool {
        state.state.chase_timer > 0
    }

    pub fn jump_mode(state: &State) -> bool {
        state.state.chase_timer > 0
    }

    pub fn jumps_remaining(state: &State) -> i32 {
        state.state.jumps
    }

    pub fn num_enemies(state: &State) -> usize {
        state.state.enemies.len()
    }

    pub fn enemy_tile(state: &State, enemy: usize) -> (i32, i32) {
        let etp = state.state.enemies[enemy].position.to_tile();
        (etp.tx, etp.ty)
    }

    pub fn enemy_caught(state: &State, enemy: usize) -> bool {
        state.state.enemies[enemy].caught
    }

    pub fn player_tile(state: &State) -> (i32, i32) {
        let tp = state.state.player.position.to_tile();
        (tp.tx, tp.ty)
    }
}

#[cfg(feature = "amidar")]
#[cfg(test)]
mod amidar_q_tests {
    use super::super::amidar;
    use super::amidar as q;
    use toybox_core::{Input, Simulation, State};

    #[test]
    fn test_q_num_tiles_unpainted() {
        let mut state = amidar::State::try_new(&amidar::Amidar::default()).unwrap();

        let (px, py) = q::player_tile(&state);
        let first = q::num_tiles_unpainted(&state);

        let mut go_up = Input::default();
        go_up.up = true;

        // Move the user up (be a little robust to how long the animation takes.)
        for _ in 0..5000 {
            state.update_mut(go_up);
            if state.state.score > 0 {
                // we must have painted something!
                break;
            }
        }

        let (nx, ny) = q::player_tile(&state);
        if py == ny {
            panic!("Player can't move upward!")
        }
        println!("Moved player to ({},{}) from ({},{})", nx, ny, px, py);

        let painted_now = q::num_tiles_unpainted(&state);
        println!("painted_now: {} ... before: {}", painted_now, first);
        assert!(painted_now < first);
    }
}

#[cfg(feature = "breakout")]
#[cfg(test)]
mod breakout_q_tests {
    use super::super::breakout;
    use toybox_core::Simulation;

    #[test]
    fn test_q_breakout_bricks_remaining() {
        let mut breakout = breakout::Breakout::default();
        let state = breakout.new_game();
        let bricks_remaining = state
            .query_json("bricks_remaining", &serde_json::Value::Null)
            .unwrap()
            .parse::<u32>()
            .unwrap();
        let num_columns = state
            .query_json("num_columns", &serde_json::Value::Null)
            .unwrap()
            .parse::<u32>()
            .unwrap();
        let num_rows = state
            .query_json("num_rows", &serde_json::Value::Null)
            .unwrap()
            .parse::<u32>()
            .unwrap();

        assert_eq!(bricks_remaining, num_columns * num_rows);
    }

    #[test]
    fn test_q_breakout_channels() {
        let mut breakout = breakout::Breakout::default();
        let state = breakout.new_game();

        let empty = state.query_json("channels", &serde_json::Value::Null).unwrap();
        assert_eq!(empty, "[]");
    }

}
