extern crate clap;
extern crate failure;
extern crate human_play;
extern crate toybox;

use std::sync::Arc;

extern crate quicksilver;
use clap::{App, Arg};
use human_play::color_convert;
use quicksilver::{
    geom::{Rectangle, Vector},
    graphics::{Color, Draw, Image, PixelFormat, View, Window, WindowBuilder},
    run,
};
use toybox::graphics::{Drawable, FixedSpriteData, ImageBuffer};

static mut GAME_ID: usize = 0;

struct AbstractGame {
    factory: Box<toybox::Simulation>,
    state: Box<toybox::State>,
    cached_images: Vec<(FixedSpriteData, Image)>,
}

impl quicksilver::State for AbstractGame {
    fn new() -> AbstractGame {
        let game_to_play = toybox::GAME_LIST[unsafe { GAME_ID }];
        let factory = toybox::get_simulation_by_name(&game_to_play)
            .unwrap_or_else(|_| panic!("We should be able to get a game for `{}`", game_to_play));
        let state = factory.new_game();
        AbstractGame {
            factory,
            state,
            cached_images: Vec::new(),
        }
    }
    fn update(&mut self, window: &mut Window) {
        let buttons = human_play::process_keys(window);
        if self.state.lives() < 0 {
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
        for (z, dw) in drawables.iter().enumerate() {
            let z = z as i32;
            match dw {
                &Drawable::Rectangle { color, x, y, w, h } => {
                    window.draw(
                        &Draw::rectangle(Rectangle::new(x, y, w, h))
                            .with_color(color_convert(color))
                            .with_z(z)
                    );
                }
                &Drawable::StaticSprite {
                    x,
                    y,
                    data: ref sprite,
                } => {
                    // TODO(jfoley); why do we suddenly need to shift these? Is this a quicksilver bug?
                    let x = x+w/2;
                    let y = y+h/2;

                    let change = if let Some((_, ref img)) = self
                        .cached_images
                        .iter_mut()
                        .find(|(toybox_image, _)| Arc::ptr_eq(&toybox_image.data, &sprite.data))
                    {
                        window.draw(&Draw::image(img, Vector::new(x, y)).with_z(z));
                        None
                    } else {
                        println!("First-render: ({},{})", x, y);
                        let mut buf = ImageBuffer::alloc(w, h);
                        buf.render(&[Drawable::sprite(0, 0, sprite.clone())]);
                        let img = Image::from_raw(&buf.data, w as u32, h as u32, PixelFormat::RGBA);
                        window.draw(&Draw::image(&img, Vector::new(x, y)).with_z(z));
                        Some((sprite.clone(), img))
                    };
                    // Keep software-rendered image around for future frames of the game!
                    if let Some(pair) = change {
                        self.cached_images.push(pair);
                    }
                }
                &Drawable::DestructibleSprite(ref sprite) => {
                    let x = sprite.x;
                    let y = sprite.y;
                    let w = sprite.width() * sprite.scale();
                    let h = sprite.height() * sprite.scale();

                    let mut buf = ImageBuffer::alloc(w, h);
                    buf.render_sprite(sprite.scale, &sprite.data);
                    let img = Image::from_raw(&buf.data, w as u32, h as u32, PixelFormat::RGBA);
                    window.draw(&Draw::image(&img, Vector::new(x, y)).with_z(z));
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
