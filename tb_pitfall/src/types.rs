// contains both things that will change over time and things that remain static in-game

pub struct Pitfall; 
pub struct StateCore;

// object declarations
pub struct Room;

pub struct Obstacle;

pub struct State { 
  pub config: Pitfall,
  pub state: StateCore
}

