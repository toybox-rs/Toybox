extern crate toybox;

use toybox::amidar;
use toybox::Input;

fn main() {
    let mut state = amidar::State::try_new().unwrap();
    for i in 0..1000 {
        let buttons = &[Input::Up];
        state.update_mut(buttons);
    }
}