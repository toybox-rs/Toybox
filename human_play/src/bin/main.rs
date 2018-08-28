extern crate clap;
extern crate failure;
extern crate human_play;
extern crate toybox;

extern crate quicksilver;
use clap::{App, Arg};
use human_play::color_convert;
use quicksilver::{
    geom::Rectangle,
    graphics::{Color, Draw, View, Window, WindowBuilder},
    run,
};
use toybox::graphics::Drawable;

static mut GAME_ID: usize = 0;

struct AbstractGame {
    factory: Box<toybox::Simulation>,
    state: Box<toybox::State>,
}

impl quicksilver::State for AbstractGame {
    fn new() -> AbstractGame {
        let game_to_play = toybox::GAME_LIST[unsafe { GAME_ID }];
        let factory = toybox::get_simulation_by_name(&game_to_play).unwrap_or_else(|_| panic!(
            "We should be able to get a game for `{}`",
            game_to_play
        ));
        let state = factory.new_game();
        AbstractGame { factory, state }
    }
    fn update(&mut self, window: &mut Window) {
        let buttons = human_play::process_keys(window);
        if self.state.game_over() {
            self.state = self.factory.new_game();
            return;
        }
        self.state.update_mut(buttons);
    }
    fn draw(&mut self, window: &mut Window) {
        let (w, h) = self.factory.game_size();
        window.set_view(View::new(Rectangle::new(0, 0, w, h)));
        window.clear(Color::black());

        let drawables = self.state.draw();
        for dw in drawables {
            match dw {
                Drawable::Rectangle { color, x, y, w, h } => {
                    window.draw(
                        &Draw::rectangle(Rectangle::new(x, y, w, h))
                            .with_color(color_convert(color)),
                    );
                }
                Drawable::Sprite(sprite) => {
                    let x = sprite.x;
                    let y = sprite.y;
                    let w = sprite.width() * sprite.scale();
                    let h = sprite.height() * sprite.scale();
                    if let Some(color) = sprite.find_visible_color() {
                        window.draw(
                            &Draw::rectangle(Rectangle::new(x, y, w, h))
                                .with_color(color_convert(color)),
                        );
                    }
                }
            }
        }

        window.present();
    }
}
fn main() {
    let matches = App::new("human_play")
        .arg(
            Arg::with_name("game")
                .long("game")
                .value_name("GAME")
                .help("Try amidar, breakout or space_invaders. (amidar by default)")
                .takes_value(true),
        ).get_matches();

    let game = matches.value_of("game").unwrap_or("amidar");
    let game_num = toybox::GAME_LIST
        .iter()
        .position(|gn| gn == &game)
        .expect("Couldn't find your game!");
    unsafe {
        GAME_ID = game_num;
    }
    let factory = toybox::get_simulation_by_name(&game).unwrap();
    let (w, h) = factory.game_size();
    run::<AbstractGame>(WindowBuilder::new("Toybox::human_play", w as u32, h as u32));
}
