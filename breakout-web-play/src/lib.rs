extern crate cfg_if;
extern crate wasm_bindgen;
extern crate toybox;

mod utils;

use cfg_if::cfg_if;
use wasm_bindgen::prelude::*;

use toybox::graphics::{Drawable, FixedSpriteData, ImageBuffer, Color};
use toybox::{Input, State, Simulation};

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
extern {
    fn alert(s: &str);
    #[wasm_bindgen(js_namespace=console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("Welcome to toybox's breakout-web-play!");

    let mut factory = toybox::get_simulation_by_name("breakout")
        .expect("We should be able to get the breakout game");

    let state = factory.new_game();
    log(&state.to_json());

}
