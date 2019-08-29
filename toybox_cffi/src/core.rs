use super::WrapSimulator;
use super::WrapState;
use libc::{c_char, c_void};
use serde_json;
use std::boxed::Box;
use std::ffi::{CStr, CString};
use std::mem;
use toybox;
use toybox_core::graphics::{GrayscaleBuffer, ImageBuffer};
use toybox_core::{AleAction, Input, State};

#[no_mangle]
pub extern "C" fn free_str(originally_from_rust: *mut c_char) {
    let _will_drop: CString = unsafe { CString::from_raw(originally_from_rust) };
}

#[no_mangle]
pub extern "C" fn simulator_alloc(name: *const c_char) -> *mut WrapSimulator {
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

#[no_mangle]
pub extern "C" fn simulator_to_json(ptr: *mut WrapSimulator) -> *const c_char {
    let &mut WrapSimulator { ref mut simulator } = unsafe {
        assert!(!ptr.is_null());
        &mut *ptr
    };

    let json: String = simulator.to_json();
    let cjson: CString = CString::new(json).expect("Simulator JSON &str to CString fail!");
    CString::into_raw(cjson)
}

#[no_mangle]
pub extern "C" fn simulator_is_legal_action(ptr: *mut WrapSimulator, action: i32) -> bool {
    let &mut WrapSimulator { ref mut simulator } = unsafe {
        assert!(!ptr.is_null());
        &mut *ptr
    };
    let actions = simulator.legal_action_set();
    if let Some(action) = AleAction::from_int(action) {
        actions.contains(&action)
    } else {
        false
    }
}

#[no_mangle]
pub extern "C" fn simulator_actions(ptr: *mut WrapSimulator) -> *const c_char {
    let &mut WrapSimulator { ref mut simulator } = unsafe {
        assert!(!ptr.is_null());
        &mut *ptr
    };
    let actions: Vec<i32> = simulator
        .legal_action_set()
        .into_iter()
        .map(|a| a.to_int())
        .collect();
    let actions = serde_json::to_string(&actions).expect("Vector to JSON should be OK.");
    let cjson: CString = CString::new(actions).expect("Conversion to CString should succeed!");
    CString::into_raw(cjson)
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

/// Hopefully the last "query" cbinding we need to write.
#[no_mangle]
pub extern "C" fn state_query_json(
    ptr: *mut WrapState,
    query_str: *const c_char,
    args_json_str: *const c_char,
) -> *const c_char {
    // Validate state pointer.
    let &mut WrapState { ref mut state } = unsafe {
        assert!(!ptr.is_null());
        &mut *ptr
    };
    // Validate query string pointer.
    let query_str: &CStr = unsafe { CStr::from_ptr(query_str) };
    let query_str: &str = query_str
        .to_str()
        .expect("Could not convert your query string to UTF-8!");
    // Validate args json string pointer.
    let args_str: &CStr = unsafe { CStr::from_ptr(args_json_str) };
    let args_str: &str = args_str
        .to_str()
        .expect("Could not convert your args json string to UTF-8!");
    let args: serde_json::Value =
        serde_json::from_str(args_str).expect("Could not convert your args string to JSON!");

    let json_str = match state.query_json(query_str, &args) {
        Ok(s) => s,
        Err(qe) => format!("{:?}", qe),
    };
    let cjson: CString = CString::new(json_str).expect("Conversion to CString should succeed!");
    CString::into_raw(cjson)
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
pub extern "C" fn state_apply_ale_action(state_ptr: *mut WrapState, input: i32) -> bool {
    let &mut WrapState { ref mut state } = unsafe {
        assert!(!state_ptr.is_null());
        &mut *state_ptr
    };
    if let Some(input) = AleAction::from_int(input).map(|a| a.to_input()) {
        state.update_mut(input);
        true
    } else {
        false
    }
}

#[no_mangle]
pub extern "C" fn state_apply_action(state_ptr: *mut WrapState, input_ptr: *const c_char) {
    let &mut WrapState { ref mut state } = unsafe {
        assert!(!state_ptr.is_null());
        &mut *state_ptr
    };
    let input_ptr = unsafe {
        assert!(!input_ptr.is_null());
        CStr::from_ptr(input_ptr)
    };
    let input_str = input_ptr
        .to_str()
        .expect("Could not create input string from pointer");
    let input: Input = serde_json::from_str(input_str).expect("Could not input string to Input");
    state.update_mut(input);
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
pub extern "C" fn state_to_json(state_ptr: *mut WrapState) -> *mut c_void {
    let &mut WrapState { ref mut state } = unsafe {
        assert!(!state_ptr.is_null());
        &mut *state_ptr
    };

    let json: String = state.to_json();
    let cjson: CString = CString::new(json).expect("Conversion to CString should succeed!");
    CString::into_raw(cjson) as *mut c_void
}

#[no_mangle]
pub extern "C" fn state_from_json(
    ptr: *mut WrapSimulator,
    json_str: *const c_char,
) -> *mut WrapState {
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

#[no_mangle]
pub extern "C" fn simulator_from_json(
    ptr: *mut WrapSimulator,
    json_str: *const c_char,
) -> *mut WrapSimulator {
    let json_str: &CStr = unsafe { CStr::from_ptr(json_str) };
    let json_str: &str = json_str
        .to_str()
        .expect("Could not convert your config string to UTF-8!");

    let &mut WrapSimulator { ref mut simulator } = unsafe {
        assert!(!ptr.is_null());
        &mut *ptr
    };
    let new_sim = simulator
        .from_json(json_str)
        .expect("Could not parse some JSON!");

    let out = Box::new(WrapSimulator { simulator: new_sim });
    Box::into_raw(out)
}
