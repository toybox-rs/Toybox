// Contains predicates that are useful for interventions.
// These can all be implemented over json, but exporting some fast
// ones from Rust will be helpful.

#[cfg(feature = "amidar")]
#[cfg(test)]
mod amidar_q_tests {
    use super::super::amidar;
    use toybox_core::{Input, Simulation, State};

    fn player_tile(state: &State) -> (i32, i32) {
        serde_json::from_str(
            &state
                .query_json("player_tile", &serde_json::Value::Null)
                .unwrap(),
        )
        .unwrap()
    }
    fn num_tiles_unpainted(state: &State) -> usize {
        serde_json::from_str(
            &state
                .query_json("num_tiles_unpainted", &serde_json::Value::Null)
                .unwrap(),
        )
        .unwrap()
    }

    #[test]
    fn test_q_num_tiles_unpainted() {
        let mut state = amidar::State::try_new(&amidar::Amidar::default()).unwrap();

        let (px, py) = player_tile(&state);
        let first = num_tiles_unpainted(&state);

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

        let (nx, ny) = player_tile(&state);
        if py == ny {
            panic!("Player can't move upward!")
        }
        println!("Moved player to ({},{}) from ({},{})", nx, ny, px, py);

        let painted_now = num_tiles_unpainted(&state);
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

        let empty = state
            .query_json("channels", &serde_json::Value::Null)
            .unwrap();
        assert_eq!(empty, "[]");
    }

}
