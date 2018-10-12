extern crate clap;
extern crate failure;
extern crate toybox;

use toybox::graphics::{Drawable};
use toybox::Input;

use clap::{App, Arg};

fn main() {
    let matches = App::new("headless-simulation")
        .arg(
            Arg::with_name("game")
                .long("game")
                .value_name("GAME")
                .help("Try amidar, breakout or space_invaders. (amidar by default)")
                .takes_value(true),
        ).get_matches();

    let game = matches.value_of("game").unwrap_or("amidar");

    let mut simulator = toybox::get_simulation_by_name(game).unwrap();
    let (w, h) = simulator.game_size();
    let mut state = simulator.new_game();

    println!("<?xml version=\"1.0\" encoding=\"UTF-8\" ?>");
    println!("<svg width=\"{}\" height=\"{}\" xmlns=\"http://www.w3.org/2000/svg\" xmlns:xlink=\"http://www.w3.org/1999/xlink\">", w, h);
    for dw in state.draw().into_iter() {
        match dw {
                Drawable::Rectangle { id, color, x, y, w, h } => {
                    print!("  <rect x=\"{}\" y=\"{}\" width=\"{}\" height=\"{}\" style=\"fill:rgb({},{},{})\" ", x, y, w, h, color.r, color.g, color.b);
                    if let Some(id) = id {
                        print!("id=\"u{}\" ", id);
                    }              
                    println!(" />");
                },
                _ => {},
        }
    }
    println!("</svg>");

    let mut buttons = Input::default();
    buttons.up = true;
    buttons.button1 = true;
    state.update_mut(buttons)

}
