extern crate toybox;

use toybox::Vec2D;

const GameSize: (i32,i32) = (160, 160);

struct BreakoutState {
    ball_pos: Vec2D,
    ball_velocity: Vec2D,
    paddle_pos: Vec2D,
}

impl BreakoutState {
    fn new() -> BreakoutState {
        let seed = [13 as u8; 16];
        BreakoutState { 
            ball_pos: Vec2D::origin(), 
            ball_velocity: Vec2D::from_polar(4.0, 0.5),
            paddle_pos: Vec2D::origin() 
        }
    }
}

fn main() {

}