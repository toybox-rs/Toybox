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
    pub fn is_empty(self) -> bool {
        !self.left && !self.right && !self.up && !self.down && !self.button1 && !self.button2
    }
}
