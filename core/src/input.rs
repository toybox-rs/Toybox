/// Think NES-style controls: directions, and two buttons.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[repr(C)]
pub struct Input {
    pub left: bool,
    pub right: bool,
    pub up: bool,
    pub down: bool,
    pub button1: bool,
    pub button2: bool,
}

impl Default for Input {
    fn default() -> Self {
        Input {
            left: false,
            right: false,
            up: false,
            down: false,
            button1: false,
            button2: false,
        }
    }
}
impl Input {
    pub fn new() -> Input {
        Input::default()
    }
    pub fn new_from_ale(x: i32) -> Option<Input> {
        AleAction::from_int(x).map(|a| a.to_input())
    }
    pub fn is_empty(self) -> bool {
        !self.left && !self.right && !self.up && !self.down && !self.button1 && !self.button2
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, PartialOrd, Ord)]
pub enum AleAction {
    NOOP,
    FIRE,
    UP,
    RIGHT,
    LEFT,
    DOWN,
    UPRIGHT,
    UPLEFT,
    DOWNRIGHT,
    DOWNLEFT,
    UPFIRE,
    RIGHTFIRE,
    LEFTFIRE,
    DOWNFIRE,
    UPRIGHTFIRE,
    UPLEFTFIRE,
    DOWNRIGHTFIRE,
    DOWNLEFTFIRE,
}

impl AleAction {
    pub fn to_int(self) -> i32 {
        match self {
            AleAction::NOOP => 0,
            AleAction::FIRE => 1,
            AleAction::UP => 2,
            AleAction::RIGHT => 3,
            AleAction::LEFT => 4,
            AleAction::DOWN => 5,
            AleAction::UPRIGHT => 6,
            AleAction::UPLEFT => 7,
            AleAction::DOWNRIGHT => 8,
            AleAction::DOWNLEFT => 9,
            AleAction::UPFIRE => 10,
            AleAction::RIGHTFIRE => 11,
            AleAction::LEFTFIRE => 12,
            AleAction::DOWNFIRE => 13,
            AleAction::UPRIGHTFIRE => 14,
            AleAction::UPLEFTFIRE => 15,
            AleAction::DOWNRIGHTFIRE => 16,
            AleAction::DOWNLEFTFIRE => 17,
        }
    }
    pub fn from_int(x: i32) -> Option<AleAction> {
        match x {
            0 => Some(AleAction::NOOP),
            1 => Some(AleAction::FIRE),
            2 => Some(AleAction::UP),
            3 => Some(AleAction::RIGHT),
            4 => Some(AleAction::LEFT),
            5 => Some(AleAction::DOWN),
            6 => Some(AleAction::UPRIGHT),
            7 => Some(AleAction::UPLEFT),
            8 => Some(AleAction::DOWNRIGHT),
            9 => Some(AleAction::DOWNLEFT),
            10 => Some(AleAction::UPFIRE),
            11 => Some(AleAction::RIGHTFIRE),
            12 => Some(AleAction::LEFTFIRE),
            13 => Some(AleAction::DOWNFIRE),
            14 => Some(AleAction::UPRIGHTFIRE),
            15 => Some(AleAction::UPLEFTFIRE),
            16 => Some(AleAction::DOWNRIGHTFIRE),
            17 => Some(AleAction::DOWNLEFTFIRE),
            _ => None,
        }
    }
    pub fn to_input(self) -> Input {
        let mut input = Input::default();
        match self {
            AleAction::NOOP => {}
            AleAction::FIRE => {
                input.button1 = true;
            }
            AleAction::UP => {
                input.up = true;
            }
            AleAction::RIGHT => {
                input.right = true;
            }
            AleAction::LEFT => {
                input.left = true;
            }
            AleAction::DOWN => {
                input.down = true;
            }
            AleAction::UPRIGHT => {
                input.up = true;
                input.right = true;
            }
            AleAction::UPLEFT => {
                input.up = true;
                input.left = true;
            }
            AleAction::DOWNRIGHT => {
                input.down = true;
                input.right = true;
            }
            AleAction::DOWNLEFT => {
                input.down = true;
                input.left = true;
            }
            AleAction::UPFIRE => {
                input.up = true;
                input.button1 = true;
            }
            AleAction::RIGHTFIRE => {
                input.right = true;
                input.button1 = true;
            }
            AleAction::LEFTFIRE => {
                input.left = true;
                input.button1 = true;
            }
            AleAction::DOWNFIRE => {
                input.down = true;
                input.button1 = true;
            }
            AleAction::UPRIGHTFIRE => {
                input.up = true;
                input.right = true;
                input.button1 = true;
            }
            AleAction::UPLEFTFIRE => {
                input.up = true;
                input.left = true;
                input.button1 = true;
            }
            AleAction::DOWNRIGHTFIRE => {
                input.down = true;
                input.right = true;
                input.button1 = true;
            }
            AleAction::DOWNLEFTFIRE => {
                input.down = true;
                input.left = true;
                input.button1 = true;
            }
        };
        input
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_ale_to_from_int() {
        for i in 0..=17 {
            let action = AleAction::from_int(i);
            let action_int = action.map(|a| a.to_int());
            assert_eq!(Some(i), action_int);
        }
    }
}
