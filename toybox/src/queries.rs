// Contains predicates that are useful for interventions.
// These can all be implemented over json, but exporting some fast
// ones from Rust will be helpful.

pub mod amidarpreds  {
}

pub mod breakoutpreds {
    use super::super::breakout::{State, screen};

    pub fn bricks_remaining(state : &State) -> i32 {
        state.bricks.iter().filter(|b| b.alive).count() as i32
    }

    /// Returns a number corresponding to the bitmask of 
    /// the stacks that are channels. The leftmost stack is the 
    /// lowest bit
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
}