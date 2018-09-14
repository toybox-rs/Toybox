#![crate_type = "dylib"]

extern crate failure;
extern crate libc;
extern crate toybox;


use toybox::{Simulation, State};
use toybox::graphics::{render_to_buffer, ImageBuffer};
use std::boxed::Box;
use std::ffi::CStr;

#[repr(C)]
pub struct WrapSimulator {
    pub simulator : Box<Simulation>
}

#[repr(C)]
pub struct WrapState {
    pub state : Box<State>
}


#[no_mangle]
pub extern "C" fn alloc_game_simulator(name: *const i8) -> *mut WrapSimulator {
    let name : &CStr = unsafe { CStr::from_ptr(name) };
    let name : &str = name.to_str().expect("poop!");
    let simulator = toybox::get_simulation_by_name(name)
        .unwrap();
    // The boxing stuff ensures the pointer remains allocated after 
    // we leave this scope.
    let simulator = Box::new(WrapSimulator { simulator });
    Box::into_raw(simulator)
}

#[no_mangle]
pub extern "C" fn free_game_simulator(ptr: *mut WrapSimulator) {
    if ptr.is_null() {
        return;
    }
    unsafe {
        Box::from_raw(ptr);
    }
    println!("Freed the simulator...")
}

// STATE ALLOC + FREE
#[no_mangle]
pub extern "C" fn alloc_game_state(ptr: *mut WrapSimulator) -> *mut WrapState {
    let WrapSimulator { simulator } = unsafe {
        assert!(!ptr.is_null());
        &mut *ptr
    };
    let state : Box<State> = simulator.new_game();
    let boxed_wrapped_state : Box<WrapState> = Box::new(WrapState { state });
    Box::into_raw(boxed_wrapped_state)
}

#[no_mangle]
pub extern "C" fn free_game_state(ptr: *mut WrapState) {
    if ptr.is_null() {
        return;
    }
    unsafe {
        Box::from_raw(ptr);
    }
    println!("Freed the state...")
}

// Need this information to initialize the numpy array in python
#[no_mangle]
pub extern "C" fn frame_width(ptr: *mut WrapSimulator) -> i32 {
    let WrapSimulator { simulator } = unsafe { 
        assert!(!ptr.is_null());
        &mut *ptr
    };
    let (w, _) = simulator.game_size();
    w
}

#[no_mangle]
pub extern "C" fn frame_height(ptr: *mut WrapSimulator) -> i32 {
    let WrapSimulator { simulator } = unsafe {
        assert!(!ptr.is_null());
        &mut *ptr
    };
    let (_, h) = simulator.game_size();
    h
}


#[no_mangle]
pub extern "C" fn render_current_frame(
    numpy_pixels: *mut u8, 
    numpy_pixels_len: usize,
    sim_ptr: *mut WrapSimulator, 
    state_ptr: *mut WrapState) {

    let WrapSimulator { simulator } = unsafe {
        assert!(!sim_ptr.is_null());
        &mut *sim_ptr
    };
    let WrapState { state } = unsafe {
        assert!(!state_ptr.is_null());
        &mut *state_ptr
    };
    let (w, h) = simulator.game_size();
    let mut img = ImageBuffer::alloc(w, h);
    render_to_buffer(&mut img, &state.draw());

    let mut dat = unsafe {
        Vec::from_raw_parts(numpy_pixels, numpy_pixels_len, numpy_pixels_len)
    };

    for i in 0..w {
        for j in 0..h {
            let k = (j * w + i) as usize;
            dat[k] = img.data[k];
        }
    }
    std::mem::forget(dat)
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
