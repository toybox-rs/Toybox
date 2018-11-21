use super::WrapState;
use amidar;
use toybox::queries::amidar::*;

#[no_mangle]
pub extern "C" fn amidar_num_tiles_unpainted(state_ptr: *mut WrapState) -> i32 {
    let &mut WrapState { ref mut state } = unsafe {
        assert!(!state_ptr.is_null());
        &mut *state_ptr
    };

    let amidar: &amidar::State = state
        .as_any()
        .downcast_ref()
        .expect("Requires amidar State for num_tiles_unpainted.");
    num_tiles_unpainted(amidar)
}

#[no_mangle]
pub extern "C" fn amidar_num_enemies(state_ptr: *mut WrapState) -> i32 {
    let &mut WrapState { ref mut state } = unsafe {
        assert!(!state_ptr.is_null());
        &mut *state_ptr
    };

    let amidar: &amidar::State = state
        .as_any()
        .downcast_ref()
        .expect("Requires amidar State for num_enemies.");
    num_enemies(amidar) as i32
}

#[no_mangle]
pub extern "C" fn amidar_jumps_remaining(state_ptr: *mut WrapState) -> i32 {
    let &mut WrapState { ref mut state } = unsafe {
        assert!(!state_ptr.is_null());
        &mut *state_ptr
    };

    let amidar: &amidar::State = state
        .as_any()
        .downcast_ref()
        .expect("Requires amidar State for jumps_remaining.");
    jumps_remaining(amidar) as i32
}

#[no_mangle]
pub extern "C" fn amidar_regular_mode(state_ptr: *mut WrapState) -> bool {
    let &mut WrapState { ref mut state } = unsafe {
        assert!(!state_ptr.is_null());
        &mut *state_ptr
    };

    let amidar: &amidar::State = state
        .as_any()
        .downcast_ref()
        .expect("Requires amidar State for regular_mode.");
    regular_mode(amidar)
}

#[no_mangle]
pub extern "C" fn amidar_chase_mode(state_ptr: *mut WrapState) -> bool {
    let &mut WrapState { ref mut state } = unsafe {
        assert!(!state_ptr.is_null());
        &mut *state_ptr
    };

    let amidar: &amidar::State = state
        .as_any()
        .downcast_ref()
        .expect("Requires amidar State for chase_mode.");
    chase_mode(amidar)
}

#[no_mangle]
pub extern "C" fn amidar_jump_mode(state_ptr: *mut WrapState) -> bool {
    let &mut WrapState { ref mut state } = unsafe {
        assert!(!state_ptr.is_null());
        &mut *state_ptr
    };

    let amidar: &amidar::State = state
        .as_any()
        .downcast_ref()
        .expect("Requires amidar State for jump_mode.");
    jump_mode(amidar)
}

#[no_mangle]
pub extern "C" fn amidar_player_tile_x(state_ptr: *mut WrapState) -> i32 {
    let &mut WrapState { ref mut state } = unsafe {
        assert!(!state_ptr.is_null());
        &mut *state_ptr
    };

    let amidar: &amidar::State = state
        .as_any()
        .downcast_ref()
        .expect("Requires amidar State for player_tile_x.");
    let (x, _) = player_tile(amidar);
    x
}

#[no_mangle]
pub extern "C" fn amidar_player_tile_y(state_ptr: *mut WrapState) -> i32 {
    let &mut WrapState { ref mut state } = unsafe {
        assert!(!state_ptr.is_null());
        &mut *state_ptr
    };

    let amidar: &amidar::State = state
        .as_any()
        .downcast_ref()
        .expect("Requires amidar State for player_tile_y.");
    let (_, y) = player_tile(amidar);
    y
}

#[no_mangle]
pub extern "C" fn amidar_enemy_tile_x(state_ptr: *mut WrapState, enemy_id: i32) -> i32 {
    let &mut WrapState { ref mut state } = unsafe {
        assert!(!state_ptr.is_null());
        &mut *state_ptr
    };

    let amidar: &amidar::State = state
        .as_any()
        .downcast_ref()
        .expect("Requires amidar State for enemy_tile_x.");
    let (x, _) = enemy_tile(amidar, enemy_id as usize);
    x
}

#[no_mangle]
pub extern "C" fn amidar_enemy_tile_y(state_ptr: *mut WrapState, enemy_id: i32) -> i32 {
    let &mut WrapState { ref mut state } = unsafe {
        assert!(!state_ptr.is_null());
        &mut *state_ptr
    };

    let amidar: &amidar::State = state
        .as_any()
        .downcast_ref()
        .expect("Requires amidar State for enemy_tile_y.");
    let (_, y) = enemy_tile(amidar, enemy_id as usize);
    y
}

#[no_mangle]
pub extern "C" fn amidar_enemy_caught(state_ptr: *mut WrapState, enemy_id: i32) -> bool {
    let &mut WrapState { ref mut state } = unsafe {
        assert!(!state_ptr.is_null());
        &mut *state_ptr
    };

    let amidar: &amidar::State = state
        .as_any()
        .downcast_ref()
        .expect("Requires amidar State for enemy_caught.");
    enemy_caught(amidar, enemy_id as usize)
}
