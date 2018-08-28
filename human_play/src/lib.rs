extern crate failure;
extern crate toybox;

use failure::Error;

extern crate quicksilver;
use quicksilver::{
    geom::{Rectangle, Vector},
    graphics::{Color, Draw, Font, View, Window, WindowBuilder},
    input::Key,
    run, Future, State,
};
use toybox::Input;

pub fn process_keys(window: &Window) -> Vec<Input> {
    let keys = window.keyboard();
    let mut buttons = Vec::new();

    if keys[Key::Up].is_down() || keys[Key::W].is_down() {
        buttons.push(Input::Up);
    }
    if keys[Key::Down].is_down() || keys[Key::S].is_down() {
        buttons.push(Input::Down);
    }
    if keys[Key::Left].is_down() || keys[Key::A].is_down() {
        buttons.push(Input::Left);
    }
    if keys[Key::Right].is_down() || keys[Key::D].is_down() {
        buttons.push(Input::Right);
    }
    if keys[Key::Z].is_down() || keys[Key::Space].is_down() {
        buttons.push(Input::Button1);
    }
    if keys[Key::X].is_down() {
        buttons.push(Input::Button2);
    }

    buttons
}