#![crate_type = "dylib"]

extern crate failure;
extern crate libc;
extern crate toybox;


use toybox::{Simulation, State};
use toybox::graphics::{ImageBuffer, GrayscaleBuffer};
use toybox::Input;
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
pub extern "C" fn simulator_alloc(name: *const i8) -> *mut WrapSimulator {
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
pub extern "C" fn simulator_free(ptr: *mut WrapSimulator) {
    if ptr.is_null() {
        return;
    }
    unsafe {
        Box::from_raw(ptr);
    }
}

// STATE ALLOC + FREE
#[no_mangle]
pub extern "C" fn state_alloc(ptr: *mut WrapSimulator) -> *mut WrapState {
    let WrapSimulator { simulator } = unsafe {
        assert!(!ptr.is_null());
        &mut *ptr
    };
    let state : Box<State> = simulator.new_game();
    let boxed_wrapped_state : Box<WrapState> = Box::new(WrapState { state });
    Box::into_raw(boxed_wrapped_state)
}

#[no_mangle]
pub extern "C" fn state_free(ptr: *mut WrapState) {
    if ptr.is_null() {
        return;
    }
    unsafe {
        Box::from_raw(ptr);
    }
}

// Need this information to initialize the numpy array in python
#[no_mangle]
pub extern "C" fn simulator_frame_width(ptr: *mut WrapSimulator) -> i32 {
    let WrapSimulator { simulator } = unsafe { 
        assert!(!ptr.is_null());
        &mut *ptr
    };
    let (w, _) = simulator.game_size();
    w
}

#[no_mangle]
pub extern "C" fn simulator_frame_height(ptr: *mut WrapSimulator) -> i32 {
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
    grayscale: bool,
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

    let imgdata = if grayscale {
        let mut img = GrayscaleBuffer::alloc(w, h);
        img.render(&state.draw());
        img.data
    } else {
        let mut img = ImageBuffer::alloc(w, h);
        img.render(&state.draw());
        img.data
    };

    let mut dat: Vec<u8> = unsafe {
        Vec::from_raw_parts(numpy_pixels, 0, numpy_pixels_len)
    };
    assert_eq!(numpy_pixels_len, imgdata.len());
    assert_eq!(dat.len(), 0);
    // Copy pixels at once (let LLVM/Rust optimize it as a linear copy).
    dat.extend(&imgdata);
    assert_eq!(dat.len(), imgdata.len());
    std::mem::forget(dat)
}

#[no_mangle]
pub extern "C" fn state_apply_action(state_ptr: *mut WrapState, input_ptr: *mut Input) {
    let WrapState { state } = unsafe {
        assert!(!state_ptr.is_null());
        &mut *state_ptr
    };
    let input = unsafe { 
        assert!(!input_ptr.is_null());
        &mut *input_ptr
    };
    state.update_mut(*input);
}

#[no_mangle]
pub extern "C" fn state_lives(state_ptr: *mut WrapState) -> i32 {
    let WrapState { state } = unsafe {
        assert!(!state_ptr.is_null());
        &mut *state_ptr
    };
    state.lives()
}

#[no_mangle]
pub extern "C" fn state_score(state_ptr: *mut WrapState) -> i32 {
    let WrapState { state } = unsafe {
        assert!(!state_ptr.is_null());
        &mut *state_ptr
    };
    state.score()
}

// fn simulate_n_frames(game_state: &mut State, n: u32) {
//   for _ in range(n) {
//     game_state.update_mut();
//   }
// }

// // Going to need score() on State (abstractly) so we can calculate reward in python-land.


// void simulate_n_frames(void* game_state, unsigned int n);
