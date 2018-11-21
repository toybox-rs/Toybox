extern crate clap;
extern crate failure;
extern crate toybox;
extern crate core;

use std::sync::Arc;

extern crate quicksilver;
use clap::{App, Arg};
use quicksilver::{
    geom::{Rectangle, Vector},
    graphics::{Color, Draw, Image, PixelFormat, View, Window, WindowBuilder},
    input::Key,
    run,
};
use toybox::graphics::{Color as TColor, Drawable, FixedSpriteData, ImageBuffer};
use toybox::Input;

use std::fs::File;
use core::{ImageBuffer, render_to_buffer};

static mut GAME_ID: usize = 0;

struct AbstractGame {
    factory: Box<toybox::Simulation>,
    state: Box<toybox::State>,
    cached_images: Vec<(FixedSpriteData, Image)>,
}

pub fn write_image(game : &AbstractGame) -> std::io::Result<()> {
    let drawable = game.state.draw();
    // make sure that game_size returns w, h and not h, w
    let (width, height): (i32, i32) = game.factory.game_size();
    let img = ImageBuffer::alloc(width, height);
    render_to_buffer(img, drawable);
    
    let mut file = File::open("frame.txt")?;
    file.write_all(img)?;
    Ok(())
}

impl quicksilver::State for AbstractGame {
    fn new() -> AbstractGame {
        let game_to_play = toybox::GAME_LIST[unsafe { GAME_ID }];
        let mut factory = toybox::get_simulation_by_name(&game_to_play)
            .unwrap_or_else(|_| panic!("We should be able to get a game for `{}`", game_to_play));
        let state = factory.new_game();
        AbstractGame {
            factory,
            state,
            cached_images: Vec::new(),
        }
    }
    fn update(&mut self, window: &mut Window) {
        let buttons = process_keys(window, self.state.as_ref());
        if self.state.lives() < 0 {
            self.state = self.factory.new_game();
            return;
        }
        self.state.update_mut(buttons);
        // save the the current image after drawing. Later we will want to record in batches.
        // Hard-code the file for now. 
        // Will want to save the action taken later.
        write_image(self);
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
                            .with_z(z),
                    );
                }
                &Drawable::StaticSprite {
                    x,
                    y,
                    data: ref sprite,
                } => {
                    // TODO(jfoley); why do we suddenly need to shift these? Is this a quicksilver bug?
                    let x = x + w / 2;
                    let y = y + h / 2;

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
        )
        .arg(
            Arg::with_name("--record")
                .takes_value(false),
        )
        .get_matches();

    let game = matches.value_of("game").unwrap_or("amidar");
    let record = matches.value_of("record").unwrap_or(true); // TODO: change this to false once I am sure it is working.
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

pub fn process_keys(window: &Window, state: &toybox::State) -> Input {
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

    if keys[Key::J].is_down() {
        println!("{}", state.to_json());
    }

    buttons
}

pub fn color_convert(color: TColor) -> Color {
    Color {
        r: color.r as f32 / 255.0,
        g: color.g as f32 / 255.0,
        b: color.b as f32 / 255.0,
        a: 1.0,
    }
}
