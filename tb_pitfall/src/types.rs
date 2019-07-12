// contains both things that will change over time and things that remain static in-game

///
#[derive(Debug,Clone, Serialize, Deserialize)]
pub struct Pitfall; 

#[derive(Debug,Clone, Serialize, Deserialize)]
pub struct StateCore{
  pub lives: i32,
  pub player: Player,
}

// object declarations
#[derive(Debug,Clone, Serialize, Deserialize)]
pub struct Room;

#[derive(Debug,Clone, Serialize, Deserialize)]
pub struct Obstacle;

#[derive(Debug,Clone, Serialize, Deserialize)]
pub struct State { 
  pub config: Pitfall,
  pub state: StateCore
}

#[derive(Debug,Clone, Serialize, Deserialize)]
pub struct Player {
  pub x: i64,
  pub y: i64,
}
