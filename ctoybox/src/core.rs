use super::WrapSimulator;
use super::WrapState;
use libc::c_char;
use std::boxed::Box;
use std::ffi::{CStr, CString};
use std::mem;
use toybox;
use toybox_core::graphics::{GrayscaleBuffer, ImageBuffer};
use toybox_core::Input;
use toybox_core::State;

#[no_mangle]
pub extern "C" fn simulator_alloc(name: *const i8) -> *mut WrapSimulator {
    let name: &CStr = unsafe { CStr::from_ptr(name) };
    let name: &str = name.to_str().expect("bad utf-8!");
    let simulator = toybox::get_simulation_by_name(name).unwrap();
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

// Reset the simulator RNG to a given seed.
#[no_mangle]
pub extern "C" fn simulator_seed(ptr: *mut WrapSimulator, seed: u32) {
    let &mut WrapSimulator { ref mut simulator } = unsafe {
        assert!(!ptr.is_null());
        &mut *ptr
    };
    simulator.reset_seed(seed);
}

// STATE ALLOC + FREE
#[no_mangle]
pub extern "C" fn state_alloc(ptr: *mut WrapSimulator) -> *mut WrapState {
    let &mut WrapSimulator { ref mut simulator } = unsafe {
        assert!(!ptr.is_null());
        &mut *ptr
    };
    let state: Box<State> = simulator.new_game();
    let boxed_wrapped_state: Box<WrapState> = Box::new(WrapState { state });
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
    let &mut WrapSimulator { ref mut simulator } = unsafe {
        assert!(!ptr.is_null());
        &mut *ptr
    };
    let (w, _) = simulator.game_size();
    w
}

#[no_mangle]
pub extern "C" fn simulator_frame_height(ptr: *mut WrapSimulator) -> i32 {
    let &mut WrapSimulator { ref mut simulator } = unsafe {
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
    state_ptr: *mut WrapState,
) {
    let &mut WrapSimulator { ref mut simulator } = unsafe {
        assert!(!sim_ptr.is_null());
        &mut *sim_ptr
    };
    let &mut WrapState { ref mut state } = unsafe {
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

    let mut dat: Vec<u8> = unsafe { Vec::from_raw_parts(numpy_pixels, 0, numpy_pixels_len) };
    assert_eq!(numpy_pixels_len, imgdata.len());
    assert_eq!(dat.len(), 0);
    // Copy pixels at once (let LLVM/Rust optimize it as a linear copy).
    dat.extend(&imgdata);
    assert_eq!(dat.len(), imgdata.len());
    mem::forget(dat)
}

#[no_mangle]
pub extern "C" fn state_apply_action(state_ptr: *mut WrapState, input_ptr: *mut Input) {
    let &mut WrapState { ref mut state } = unsafe {
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
    let &mut WrapState { ref mut state } = unsafe {
        assert!(!state_ptr.is_null());
        &mut *state_ptr
    };
    state.lives()
}

#[no_mangle]
pub extern "C" fn state_score(state_ptr: *mut WrapState) -> i32 {
    let &mut WrapState { ref mut state } = unsafe {
        assert!(!state_ptr.is_null());
        &mut *state_ptr
    };
    state.score()
}

#[no_mangle]
pub extern "C" fn to_json(state_ptr: *mut WrapState) -> *const c_char {
    let &mut WrapState { ref mut state } = unsafe {
        assert!(!state_ptr.is_null());
        &mut *state_ptr
    };

    let json: String = state.to_json();
    let cjson: CString = CString::new(json).expect("crap!");
    CString::into_raw(cjson)
}

#[no_mangle]
pub extern "C" fn from_json(ptr: *mut WrapSimulator, json_str: *const i8) -> *mut WrapState {
    let json_str: &CStr = unsafe { CStr::from_ptr(json_str) };
    let json_str: &str = json_str
        .to_str()
        .expect("Could not convert your string to UTF-8!");
    let &mut WrapSimulator { ref mut simulator } = unsafe {
        assert!(!ptr.is_null());
        &mut *ptr
    };
    let state = simulator
        .new_state_from_json(json_str)
        .expect("Could not parse state JSON!");
    let state = Box::new(WrapState { state });
    Box::into_raw(state)
}
