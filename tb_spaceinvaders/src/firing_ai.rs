use rand::Rng;
/// A collection of the possible firing protocols
use types::{SpaceInvaders, StateCore};

/// This enum represents the different enemy AI for firing in Space Invaders.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum FiringAI {
    /// The default AI trades off between aiming at the user and random firing based on config.jitter.
    TargetPlayer,
}

// Note: using the RNG makes us mutable.
fn target_player(state: &mut StateCore, config: &SpaceInvaders) -> u32 {
    let p = config.jitter;
    let r: f64 = state.rand.gen();
    assert!(r >= 0. && r < 1.);
    if r < p {
        let active_ids = state.active_weapon_enemy_ids();
        active_ids[state.rand.gen_range(0, active_ids.len()) as usize]
    } else {
        // Get active enemy closest to the player
        state.closest_enemy_id()
    }
}

pub fn enemy_fire_lasers(state: &mut StateCore, config: &SpaceInvaders) -> u32 {
    match config.enemy_protocol {
        FiringAI::TargetPlayer => target_player(state, config),
    }
}
