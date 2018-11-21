use super::WrapState;
use breakout;
use std::mem;
use toybox::queries::breakout::*;

#[no_mangle]
pub extern "C" fn breakout_brick_live_by_index(
    state_ptr: *mut WrapState,
    brick_index: usize,
) -> bool {
    let &mut WrapState { ref mut state } = unsafe {
        assert!(!state_ptr.is_null());
        &mut *state_ptr
    };

    let breakout: &breakout::State = state
        .as_any()
        .downcast_ref()
        .expect("Requires breakout State for bricks_remaining.");
    brick_live_by_index(breakout, brick_index)
}

#[no_mangle]
pub extern "C" fn breakout_bricks_remaining(state_ptr: *mut WrapState) -> i32 {
    let &mut WrapState { ref mut state } = unsafe {
        assert!(!state_ptr.is_null());
        &mut *state_ptr
    };

    let breakout: &breakout::State = state
        .as_any()
        .downcast_ref()
        .expect("Requires breakout State for bricks_remaining.");
    bricks_remaining(breakout)
}

#[no_mangle]
pub extern "C" fn breakout_num_rows(state_ptr: *mut WrapState) -> i32 {
    let &mut WrapState { ref mut state } = unsafe {
        assert!(!state_ptr.is_null());
        &mut *state_ptr
    };

    let breakout: &breakout::State = state
        .as_any()
        .downcast_ref()
        .expect("Requires breakout State for num_rows.");
    num_rows(breakout)
}

#[no_mangle]
pub extern "C" fn breakout_num_columns(state_ptr: *mut WrapState) -> i32 {
    let &mut WrapState { ref mut state } = unsafe {
        assert!(!state_ptr.is_null());
        &mut *state_ptr
    };

    let breakout: &breakout::State = state
        .as_any()
        .downcast_ref()
        .expect("Requires breakout State for num_columns.");
    num_columns(breakout)
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
    let breakout: &breakout::State = state
        .as_any()
        .downcast_ref()
        .expect("Requires breakout State for channels.");

    // Construct a temporary vector from
    let mut target: Vec<i32> =
        unsafe { Vec::from_raw_parts(numpy_channels, 0, numpy_channels_len) };

    // This is Rust's answer.
    let src = channels(breakout);

    // Your array is too small!
    if src.len() >= numpy_channels_len {
        // Don't let rust de-allocate numpy's stuff.
        mem::forget(target);
        return -1;
    };
    // Copy it in.
    target.extend(src);
    // Tell the user how many we found.
    let rc = target.len() as isize;
    // Don't let rust de-allocate numpy's stuff.
    mem::forget(target);
    // return the size
    rc
}
