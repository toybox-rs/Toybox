extern crate cfg_if;
extern crate toybox;
extern crate wasm_bindgen;

mod utils;

use cfg_if::cfg_if;
use wasm_bindgen::prelude::*;

use toybox::graphics::{GrayscaleBuffer, ImageBuffer};
use toybox::{Input, Simulation, State};

cfg_if! {
    // When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
    // allocator.
    if #[cfg(feature = "wee_alloc")] {
        extern crate wee_alloc;
        #[global_allocator]
        static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
    }
}

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
    #[wasm_bindgen(js_namespace=console)]
    fn log(s: &str);
}

#[repr(C)]
pub struct WrapSimulator {
    pub simulator: Box<Simulation>,
}

#[repr(C)]
pub struct WrapState {
    pub state: Box<State>,
}

#[wasm_bindgen]
pub struct WebInput {
    input: Input,
}

#[wasm_bindgen]
impl WebInput {
    pub fn new() -> WebInput {
        WebInput {
            input: Input::default(),
        }
    }
    pub fn set_left(&mut self, val: bool) {
        self.input.left = val;
    }
    pub fn set_right(&mut self, val: bool) {
        self.input.right = val;
    }
    pub fn set_up(&mut self, val: bool) {
        self.input.up = val;
    }
    pub fn set_down(&mut self, val: bool) {
        self.input.down = val;
    }
    pub fn set_button1(&mut self, val: bool) {
        self.input.button1 = val;
    }
    pub fn set_button2(&mut self, val: bool) {
        self.input.button2 = val;
    }
}

#[wasm_bindgen]
pub extern "C" fn simulator_alloc(name: &str) -> *mut WrapSimulator {
    let simulator = toybox::get_simulation_by_name(name).unwrap();
    // The boxing stuff ensures the pointer remains allocated after
    // we leave this scope.
    let simulator = Box::new(WrapSimulator { simulator });
    Box::into_raw(simulator)
}

#[wasm_bindgen]
pub extern "C" fn simulator_free(ptr: *mut WrapSimulator) {
    if ptr.is_null() {
        return;
    }
    unsafe {
        Box::from_raw(ptr);
    }
}

// Reset the simulator RNG to a given seed.
#[wasm_bindgen]
pub extern "C" fn simulator_seed(ptr: *mut WrapSimulator, seed: u32) {
    let &mut WrapSimulator { ref mut simulator } = unsafe {
        assert!(!ptr.is_null());
        &mut *ptr
    };
    simulator.reset_seed(seed);
}

// STATE ALLOC + FREE
#[wasm_bindgen]
pub extern "C" fn state_alloc(ptr: *mut WrapSimulator) -> *mut WrapState {
    let &mut WrapSimulator { ref mut simulator } = unsafe {
        assert!(!ptr.is_null());
        &mut *ptr
    };
    let state: Box<State> = simulator.new_game();
    let boxed_wrapped_state: Box<WrapState> = Box::new(WrapState { state });
    Box::into_raw(boxed_wrapped_state)
}

#[wasm_bindgen]
pub extern "C" fn state_free(ptr: *mut WrapState) {
    if ptr.is_null() {
        return;
    }
    unsafe {
        Box::from_raw(ptr);
    }
}

// Need this information to initialize the numpy array in python
#[wasm_bindgen]
pub extern "C" fn simulator_frame_width(ptr: *mut WrapSimulator) -> i32 {
    let &mut WrapSimulator { ref mut simulator } = unsafe {
        assert!(!ptr.is_null());
        &mut *ptr
    };
    let (w, _) = simulator.game_size();
    w
}

#[wasm_bindgen]
pub extern "C" fn simulator_frame_height(ptr: *mut WrapSimulator) -> i32 {
    let &mut WrapSimulator { ref mut simulator } = unsafe {
        assert!(!ptr.is_null());
        &mut *ptr
    };
    let (_, h) = simulator.game_size();
    h
}

#[wasm_bindgen]
pub extern "C" fn render_current_frame(
    numpy_pixels: &mut [u8],
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
    assert_eq!(numpy_pixels_len, imgdata.len());

    for (i, val) in imgdata.into_iter().enumerate() {
        numpy_pixels[i] = val;
    }
}

#[wasm_bindgen]
pub extern "C" fn state_apply_action(state_ptr: *mut WrapState, web: WebInput) {
    let &mut WrapState { ref mut state } = unsafe {
        assert!(!state_ptr.is_null());
        &mut *state_ptr
    };

    state.update_mut(web.input);
}

#[wasm_bindgen]
pub extern "C" fn state_lives(state_ptr: *mut WrapState) -> i32 {
    let &mut WrapState { ref mut state } = unsafe {
        assert!(!state_ptr.is_null());
        &mut *state_ptr
    };
    state.lives()
}

#[wasm_bindgen]
pub extern "C" fn state_score(state_ptr: *mut WrapState) -> i32 {
    let &mut WrapState { ref mut state } = unsafe {
        assert!(!state_ptr.is_null());
        &mut *state_ptr
    };
    state.score()
}

#[wasm_bindgen]
pub extern "C" fn to_json(state_ptr: *mut WrapState) -> String {
    let &mut WrapState { ref mut state } = unsafe {
        assert!(!state_ptr.is_null());
        &mut *state_ptr
    };

    state.to_json()
}

#[wasm_bindgen]
pub extern "C" fn from_json(ptr: *mut WrapSimulator, json_str: &str) -> *mut WrapState {
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

