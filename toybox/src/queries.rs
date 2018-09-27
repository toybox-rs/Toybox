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
            for row in 0..down {
                let i = row * across + offset;
                if bricks[i as usize].alive {
                    continue 
                } else if row == down {
                    retval.push(offset)
                }
            }
        }
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