extern crate toybox;

extern crate quicksilver;
use quicksilver::{
    graphics::{Color, Window},
    input::Key,
};
use toybox::Input;
use toybox::graphics::Color as TColor;

pub fn process_keys(window: &Window) -> Input {
    let keys = window.keyboard();
    let mut buttons = Input::new();

    if keys[Key::Up].is_down() || keys[Key::W].is_down() {
        buttons.up = true;
    }
    if keys[Key::Down].is_down() || keys[Key::S].is_down() {
        buttons.down = true;
    }
    if keys[Key::Left].is_down() || keys[Key::A].is_down() {
        buttons.left = true;
    }
    if keys[Key::Right].is_down() || keys[Key::D].is_down() {
        buttons.right = true;
    }
    if keys[Key::Z].is_down() || keys[Key::Space].is_down() {
        buttons.button1 = true;
    }
    if keys[Key::X].is_down() {
        buttons.button2 = true;
    }

    buttons
}

pub fn color_convert(color: &TColor) -> Color {
    Color { 
        r: color.r as f32 / 255.0,
        g: color.g as f32 / 255.0,
        b: color.b as f32 / 255.0,
        a: 1.0
    }
}
