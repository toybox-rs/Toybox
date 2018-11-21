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
    use toybox_core::{State,Simulation,Input};

    #[test]
    fn test_q_num_tiles_unpainted() {
        let mut state = amidar::State::try_new().unwrap();

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
pub mod breakout {
    use super::super::breakout::{screen, State};

    pub fn brick_live_by_index(state: &State, brick_index: usize) -> bool {
        return state.state.bricks[brick_index].alive;
    }

    pub fn bricks_remaining(state: &State) -> i32 {
        state.state.bricks.iter().filter(|b| b.alive).count() as i32
    }

    /// Returns a set of numbers corresponding to the stacks that are channels.
    pub fn channels(state: &State) -> Vec<i32> {
        let across = screen::BRICKS_ACROSS as i32;
        let down = screen::ROW_SCORES.len() as i32;
        let bricks = &state.state.bricks;
        let mut retval = Vec::new();

        for offset in 0..across {
            let all_dead = (0..down)
                .map(|row| {
                    let i = row + offset * down;
                    !bricks[i as usize].alive
                })
                .all(|c| c);
            if all_dead {
                retval.push(offset);
                assert!(retval.len() <= (across as usize));
            }
        }
        assert!(retval.len() <= (across as usize));
        retval
    }

    /// TODO: this will someday be derived from state.config
    pub fn num_columns(_state: &State) -> i32 {
        screen::BRICKS_ACROSS as i32
    }

    /// TODO: this will someday be derived from state.config
    pub fn num_rows(_state: &State) -> i32 {
        screen::ROW_SCORES.len() as i32
    }
}

#[cfg(feature = "breakout")]
#[cfg(test)]
mod breakout_q_tests {
    use super::super::breakout;
    use super::breakout as q;
    use toybox_core::Simulation;

    #[test]
    fn test_q_breakout_bricks_remaining() {
        let mut breakout = breakout::Breakout::default();
        let state = breakout.new_game();
        let state: &breakout::State = state.as_any().downcast_ref().unwrap();

        assert_eq!(
            q::bricks_remaining(state),
            q::num_columns(state) * q::num_rows(state)
        );
    }

    #[test]
    fn test_q_breakout_channels() {
        let mut breakout = breakout::Breakout::default();
        let state = breakout.new_game();
        let state: &breakout::State = state.as_any().downcast_ref().unwrap();

        let empty = q::channels(state);
        let expected: Vec<i32> = Vec::new();
        assert_eq!(empty, expected);
    }

    #[test]
    fn test_breakout_channel_layout_assumptions() {
        let mut breakout = breakout::Breakout::default();
        let state = breakout.new_game();
        let state: &breakout::State = state.as_any().downcast_ref().unwrap();

        let across = q::num_columns(state);
        let down = q::num_rows(state);

        let bricks = &state.state.bricks;
        for offset in 0..across {
            let xs: Vec<_> = (0..down)
                .map(|row| {
                    let i = row + offset * down;
                    bricks[i as usize].position.x as i32
                })
                .collect();
            for i in (1..down) {
                assert_eq!(xs[(i - 1) as usize], xs[i as usize]);
            }
        }
    }

}
