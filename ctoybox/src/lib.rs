#![crate_type = "dylib"]

extern crate failure;
extern crate libc;
extern crate toybox;

use libc::c_char;
use std::boxed::Box;
use std::ffi::{CStr, CString};
use toybox::graphics::{GrayscaleBuffer, ImageBuffer};
use toybox::queries;
use toybox::Input;
use toybox::{Simulation, State};

#[repr(C)]
pub struct WrapSimulator {
    pub simulator: Box<Simulation>,
}

#[repr(C)]
pub struct WrapState {
    pub state: Box<State>,
}

#[no_mangle]
pub extern "C" fn simulator_alloc(name: *const i8) -> *mut WrapSimulator {
    let name: &CStr = unsafe { CStr::from_ptr(name) };
    let name: &str = name.to_str().expect("poop!");
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
    std::mem::forget(dat)
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

#[no_mangle]
pub extern "C" fn breakout_brick_live_by_index(
    state_ptr: *mut WrapState,
    brick_index: usize,
) -> bool {
    let &mut WrapState { ref mut state } = unsafe {
        assert!(!state_ptr.is_null());
        &mut *state_ptr
    };

    let breakout: &toybox::breakout::State = state
        .as_any()
        .downcast_ref()
        .expect("Requires breakout State for bricks_remaining.");
    queries::breakout::brick_live_by_index(breakout, brick_index)
}

#[no_mangle]
pub extern "C" fn breakout_bricks_remaining(state_ptr: *mut WrapState) -> i32 {
    let &mut WrapState { ref mut state } = unsafe {
        assert!(!state_ptr.is_null());
        &mut *state_ptr
    };

    let breakout: &toybox::breakout::State = state
        .as_any()
        .downcast_ref()
        .expect("Requires breakout State for bricks_remaining.");
    queries::breakout::bricks_remaining(breakout)
}

#[no_mangle]
pub extern "C" fn breakout_num_rows(state_ptr: *mut WrapState) -> i32 {
    let &mut WrapState { ref mut state } = unsafe {
        assert!(!state_ptr.is_null());
        &mut *state_ptr
    };

    let breakout: &toybox::breakout::State = state
        .as_any()
        .downcast_ref()
        .expect("Requires breakout State for num_rows.");
    queries::breakout::num_rows(breakout)
}

#[no_mangle]
pub extern "C" fn breakout_num_columns(state_ptr: *mut WrapState) -> i32 {
    let &mut WrapState { ref mut state } = unsafe {
        assert!(!state_ptr.is_null());
        &mut *state_ptr
    };

    let breakout: &toybox::breakout::State = state
        .as_any()
        .downcast_ref()
        .expect("Requires breakout State for num_columns.");
    queries::breakout::num_columns(breakout)
}

/// Following C API conventions here: returns -1 if there's an error (your destination is too small) and the number of channels if it can.
#[no_mangle]
pub extern "C" fn breakout_channels(
    state_ptr: *mut WrapState,
    numpy_channels: *mut i32,
    numpy_channels_len: usize,
) -> isize {
    let &mut WrapState { ref mut state } = unsafe {
        assert!(!state_ptr.is_null());
        &mut *state_ptr
    };

    // Crash if they give us a non-breakout State.
    let breakout: &toybox::breakout::State = state
        .as_any()
        .downcast_ref()
        .expect("Requires breakout State for channels.");

    // Construct a temporary vector from
    let mut target: Vec<i32> =
        unsafe { Vec::from_raw_parts(numpy_channels, 0, numpy_channels_len) };

    // This is Rust's answer.
    let src = queries::breakout::channels(breakout);

    // Your array is too small!
    if src.len() >= numpy_channels_len {
        // Don't let rust de-allocate numpy's stuff.
        std::mem::forget(target);
        return -1;
    };
    // Copy it in.
    target.extend(src);
    // Tell the user how many we found.
    let rc = target.len() as isize;
    // Don't let rust de-allocate numpy's stuff.
    std::mem::forget(target);
    // return the size
    rc
}

#[no_mangle]
pub extern "C" fn amidar_num_tiles_unpainted(state_ptr: *mut WrapState) -> i32 {
    let &mut WrapState { ref mut state } = unsafe {
        assert!(!state_ptr.is_null());
        &mut *state_ptr
    };

    let amidar: &toybox::amidar::State = state
        .as_any()
        .downcast_ref()
        .expect("Requires amidar State for num_tiles_unpainted.");
    queries::amidar::num_tiles_unpainted(amidar)
}

#[no_mangle]
pub extern "C" fn amidar_num_enemies(state_ptr: *mut WrapState) -> i32 {
    let &mut WrapState { ref mut state } = unsafe {
        assert!(!state_ptr.is_null());
        &mut *state_ptr
    };

    let amidar: &toybox::amidar::State = state
        .as_any()
        .downcast_ref()
        .expect("Requires amidar State for num_enemies.");
    queries::amidar::num_enemies(amidar) as i32
}

#[no_mangle]
pub extern "C" fn amidar_jumps_remaining(state_ptr: *mut WrapState) -> i32 {
    let &mut WrapState { ref mut state } = unsafe {
        assert!(!state_ptr.is_null());
        &mut *state_ptr
    };

    let amidar: &toybox::amidar::State = state
        .as_any()
        .downcast_ref()
        .expect("Requires amidar State for jumps_remaining.");
    queries::amidar::jumps_remaining(amidar) as i32
}

#[no_mangle]
pub extern "C" fn amidar_regular_mode(state_ptr: *mut WrapState) -> bool {
    let &mut WrapState { ref mut state } = unsafe {
        assert!(!state_ptr.is_null());
        &mut *state_ptr
    };

    let amidar: &toybox::amidar::State = state
        .as_any()
        .downcast_ref()
        .expect("Requires amidar State for regular_mode.");
    queries::amidar::regular_mode(amidar)
}

#[no_mangle]
pub extern "C" fn amidar_chase_mode(state_ptr: *mut WrapState) -> bool {
    let &mut WrapState { ref mut state } = unsafe {
        assert!(!state_ptr.is_null());
        &mut *state_ptr
    };

    let amidar: &toybox::amidar::State = state
        .as_any()
        .downcast_ref()
        .expect("Requires amidar State for chase_mode.");
    queries::amidar::chase_mode(amidar)
}

#[no_mangle]
pub extern "C" fn amidar_jump_mode(state_ptr: *mut WrapState) -> bool {
    let &mut WrapState { ref mut state } = unsafe {
        assert!(!state_ptr.is_null());
        &mut *state_ptr
    };

    let amidar: &toybox::amidar::State = state
        .as_any()
        .downcast_ref()
        .expect("Requires amidar State for jump_mode.");
    queries::amidar::jump_mode(amidar)
}

#[no_mangle]
pub extern "C" fn amidar_player_tile_x(state_ptr: *mut WrapState) -> i32 {
    let &mut WrapState { ref mut state } = unsafe {
        assert!(!state_ptr.is_null());
        &mut *state_ptr
    };

    let amidar: &toybox::amidar::State = state
        .as_any()
        .downcast_ref()
        .expect("Requires amidar State for player_tile_x.");
    let (x, _) = queries::amidar::player_tile(amidar);
    x
}

#[no_mangle]
pub extern "C" fn amidar_player_tile_y(state_ptr: *mut WrapState) -> i32 {
    let &mut WrapState { ref mut state } = unsafe {
        assert!(!state_ptr.is_null());
        &mut *state_ptr
    };

    let amidar: &toybox::amidar::State = state
        .as_any()
        .downcast_ref()
        .expect("Requires amidar State for player_tile_y.");
    let (_, y) = queries::amidar::player_tile(amidar);
    y
}

#[no_mangle]
pub extern "C" fn amidar_enemy_tile_x(state_ptr: *mut WrapState, enemy_id: i32) -> i32 {
    let &mut WrapState { ref mut state } = unsafe {
        assert!(!state_ptr.is_null());
        &mut *state_ptr
    };

    let amidar: &toybox::amidar::State = state
        .as_any()
        .downcast_ref()
        .expect("Requires amidar State for enemy_tile_x.");
    let (x, _) = queries::amidar::enemy_tile(amidar, enemy_id as usize);
    x
}

#[no_mangle]
pub extern "C" fn amidar_enemy_tile_y(state_ptr: *mut WrapState, enemy_id: i32) -> i32 {
    let &mut WrapState { ref mut state } = unsafe {
        assert!(!state_ptr.is_null());
        &mut *state_ptr
    };

    let amidar: &toybox::amidar::State = state
        .as_any()
        .downcast_ref()
        .expect("Requires amidar State for enemy_tile_y.");
    let (_, y) = queries::amidar::enemy_tile(amidar, enemy_id as usize);
    y
}

#[no_mangle]
pub extern "C" fn amidar_enemy_caught(state_ptr: *mut WrapState, enemy_id: i32) -> bool {
    let &mut WrapState { ref mut state } = unsafe {
        assert!(!state_ptr.is_null());
        &mut *state_ptr
    };

    let amidar: &toybox::amidar::State = state
        .as_any()
        .downcast_ref()
        .expect("Requires amidar State for enemy_caught.");
    queries::amidar::enemy_caught(amidar, enemy_id as usize)
}
