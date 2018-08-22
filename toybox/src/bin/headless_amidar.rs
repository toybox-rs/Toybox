extern crate toybox;
extern crate png;

use toybox::amidar;
use toybox::graphics::{render_to_buffer, ImageBuffer};
use toybox::Input;

fn main() {
    let num_steps = 1000;
    let (w, h) = amidar::screen::GAME_SIZE;
    let mut state = amidar::State::try_new().unwrap();
    let mut images = Vec::with_capacity(num_steps);
    for i in 0..num_steps {
        let buttons = &[Input::Up];
        state.update_mut(buttons);

        let mut img = ImageBuffer::alloc(w, h);
        render_to_buffer(&mut img, &state.draw());
        images.push(img);
    }
}
