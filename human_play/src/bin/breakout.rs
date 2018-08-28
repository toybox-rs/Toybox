extern crate toybox;
extern crate human_play;

extern crate quicksilver;
use quicksilver::{
    geom::{Rectangle},
    graphics::{Color, Draw, Window, WindowBuilder, View},
    run, State,
};
use toybox::breakout;
use toybox::graphics::Drawable;
use human_play::color_convert;

struct BreakoutGame {
    state: breakout::BreakoutState,
}

impl State for BreakoutGame {
    fn new() -> BreakoutGame {
        BreakoutGame {
            state: breakout::BreakoutState::new(),
        }
    }
    fn update(&mut self, window: &mut Window) {
        let buttons = human_play::process_keys(window);
        if self.state.game_over {
            // Any key starts a new game.
            if !buttons.is_empty() {
                self.state = breakout::BreakoutState::new()
            }
            return;
        }
        self.state.update_mut(1.0, &buttons);
    }
    fn draw(&mut self, window: &mut Window) {
        let (w, h) = breakout::screen::GAME_SIZE;
        window.set_view(View::new(Rectangle::new(0, 0, w, h)));
        window.clear(Color::black());
        
        let drawables = self.state.draw();
        for dw in drawables {
            match dw {
                Drawable::Rectangle { color, x, y, w, h } => {
                    window.draw(
                        &Draw::rectangle(Rectangle::new(x, y, w, h)).with_color(color_convert(&color)),
                    );
                }
            }
        }

        window.present();
    }
}
fn main() {
    let (w, h) = breakout::screen::GAME_SIZE;
    run::<BreakoutGame>(WindowBuilder::new("Breakout", 2*w as u32, 2*h as u32));
}
