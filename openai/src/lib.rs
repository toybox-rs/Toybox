#![crate_type = "dylib"]

extern crate failure;
extern crate toybox;
use toybox::{Simulation, State};
use toybox::graphics::{render_to_buffer, ImageBuffer};
use std::boxed::Box;
use std::ffi::CStr;

// SIMULATOR ALLOC + FREE
#[no_mangle]
pub extern "C" fn alloc_game_simulator(name: *const i8) -> *mut Simulation {
    let name : &CStr = unsafe { CStr::from_ptr(name) };
    let name : &str = name.to_str().expect("poop!");
    print!("game: {}", name);
    let simulation = toybox::get_simulation_by_name(name).unwrap();
    Box::into_raw(simulation)
}

#[no_mangle]
pub extern "C" fn free_game_simulator(simulator: *mut Simulation) {
    if simulator.is_null() {
        return;
    }
    unsafe {
        Box::from_raw(simulator);
    }
}

// STATE ALLOC + FREE
#[no_mangle]
pub extern "C" fn alloc_game_state(simulator: &mut Simulation) -> *mut State {
    Box::into_raw(simulator.new_game())
}

pub extern "C" fn free_game_state(state: *mut State) {
    if state.is_null() {
        return;
    }
    unsafe {
        Box::from_raw(state);
    }
}

// Need this information to initialize the numpy array in python
#[no_mangle]
pub extern "C" fn frame_width(simulator: *mut Simulation) -> i32 {
    let simulator = unsafe { Box::from_raw(simulator) };
    let (w, _) = simulator.game_size();
    w
}

#[no_mangle]
pub extern "C" fn frame_height(simulator: &mut Simulation) -> i32 {
    let (_, h) = simulator.game_size();
    h
}


#[no_mangle]
pub extern "C" fn render_current_frame(numpy_pixels: &mut [u8], numpy_pixels_len : i32, simulator: &mut Simulation, state: &mut State) {
    let (w, h) = simulator.game_size();
    let mut img = ImageBuffer::alloc(w, h);
    render_to_buffer(&mut img, &state.draw());

    for i in 0..w {
        for j in 0..h {
            let k = (j * w + i) as usize;
            numpy_pixels[k] = img.data[k];
        }
    }
}

// void render_current_frame(void* numpy_pixels, size_t numpy_pixels_len, void* game_state, void* simulator);

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


// void simulate_n_frames(void* game_state, unsigned int n);
