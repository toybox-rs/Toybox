#![crate_type = "dylib"]

extern crate failure;
extern crate toybox;
use toybox::State;
use std::boxed::Box;

#[no_mangle]
pub extern "C" fn new_game(name: &str) -> *mut State {
    print!("game: {}", name);
    let state = toybox::get_simulation_by_name(name)
        .unwrap()
        .new_game();
    Box::into_raw(state)
}

// fn simulate_n_frames(game_state: &mut State, n: u32) {
//   for _ in range(n) {
//     game_state.update_mut();
//   }
// }

// // Going to need score() on State (abstractly) so we can calculate reward in python-land.

// fn render_current_frame(pixels_from_numpy: &mut [u8], game_state: &State, simulator &simulator) { 
//   assert!(pixels_from_numpy.is_right_size_for(simulator));
//   graphics::render_to_numpy(pixels_from_numpy, game_state.draw());
// } 


// // unsafe { Box::into_raw(simulator_by_name(...)) and std::mem::forget } (written in Rust)
// void* alloc_game_simulator(const char* which_one);
// // unsafe { let _ = Box::from_raw(which_one) }, drop takes care of freeing.
// void free_game_simulator(void* simulator);
// int simulator_width(void* simulator);
// int simulator_height(void* simulator);
// void* alloc_new_game_state(void* simulator);
// void free_game_state(void* state);
// void simulate_n_frames(void* game_state, unsigned int n);
// void render_current_frame(void* numpy_pixels, size_t numpy_pixels_len, void* game_state, void* simulator);