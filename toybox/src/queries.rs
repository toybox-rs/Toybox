// Contains predicates that are useful for interventions.
// These can all be implemented over json, but exporting some fast
// ones from Rust will be helpful.

pub mod amidar  {
}

pub mod breakout {
    use super::super::breakout::{State, screen};

    pub fn bricks_remaining(state : &State) -> i32 {
        state.bricks.iter().filter(|b| b.alive).count() as i32
    }

    /// Returns a set of numbers corresponding to the stacks that are channels. 
    pub fn channels(state : &State) -> Vec<i32> {
        let across = screen::BRICKS_ACROSS as i32;
        let down = screen::ROW_SCORES.len() as i32;
        let bricks = &state.bricks;
        let mut retval = Vec::new();

        for offset in 0..across {
            let all_dead = (0..down).map(|row| {
                let i = row + offset * down;
                !bricks[i as usize].alive
            }).all(|c| c);
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

#[cfg(test)]
mod tests {
    use super::*;
    use Simulation;
    use super::breakout as q;
    use super::super::breakout;

    #[test]
    fn test_q_breakout_bricks_remaining() {
        let mut breakout = breakout::Breakout::default();
        let state = breakout.new_game();
        let state: &breakout::State = state.as_any().downcast_ref().unwrap();

        assert_eq!(q::bricks_remaining(state), q::num_columns(state) * q::num_rows(state));
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

    fn test_breakout_channel_layout_assumptions() {
        let mut breakout = breakout::Breakout::default();
        let state = breakout.new_game();
        let state: &breakout::State = state.as_any().downcast_ref().unwrap();

        let across = q::num_columns(state);
        let down = q::num_rows(state);

        let bricks = &state.bricks;
        for offset in 0..across {
            let xs: Vec<_> = (0..down).map(|row| {
                let i = row + offset * down;
                bricks[i as usize].position.x as i32
            }).collect();
            for i in (1..down) {
                assert_eq!(xs[(i-1) as usize], xs[i as usize]);
            }
        }
    }

}