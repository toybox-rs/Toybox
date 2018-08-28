extern crate clap;
extern crate failure;
extern crate png;
extern crate toybox;

use toybox::graphics::{render_to_buffer, ImageBuffer};
use toybox::{Simulation, Input};

use clap::{App, Arg};

use png::HasParameters;

use std::collections::VecDeque;
use std::fs::File;
use std::io::BufWriter;
use std::path::Path;

fn check_output(path: &str) -> Result<(), ()> {
    let path = Path::new(path);
    if !path.exists() {
        eprintln!("output {:?} does not exist!", path);
        return Err(());
    }
    if !path.is_dir() {
        eprintln!("output {:?} is not a directory!", path);
        return Err(());
    }
    Ok(())
}

fn main() {
    let matches = App::new("headless-simulation")
        .arg(
            Arg::with_name("game")
            .long("game")
            .value_name("GAME")
            .help("Try amidar, breakout or space_invaders. (amidar by default)")
            .takes_value(true)
        )
        .arg(
            Arg::with_name("num_steps")
                .short("n")
                .long("num_steps")
                .value_name("1000")
                .help("How many steps to simulate (also how many images to output).")
                .takes_value(true),
        ).arg(
            Arg::with_name("max_frames")
                .long("max_frames")
                .value_name("all by default, try 1000")
                .help("How many frames to keep in memory")
                .takes_value(true),
        ).arg(
            Arg::with_name("frame_step")
                .short("f")
                .long("frame_step")
                .value_name("4")
                .help("How many frames to simulate per step")
                .takes_value(true),
        ).arg(
            Arg::with_name("output")
                .long("output")
                .value_name("OUTPUT_DIR")
                .help("Where to save PNG files (directory).")
                .takes_value(true),
        ).get_matches();
    
    let game = matches.value_of("game").unwrap_or("amidar");

    let num_steps = matches
        .value_of("num_steps")
        .map(|c| c.parse::<usize>().expect("--num_steps should be a number"))
        .unwrap_or(1000);
    let frame_step = matches
        .value_of("frame_steps")
        .map(|c| {
            c.parse::<usize>()
                .expect("--frame_steps should be a number")
        }).unwrap_or(4);
    let max_frames = matches
        .value_of("max_frames")
        .map(|c| c.parse::<usize>().expect("--max_frames should be a number"));

    if let Some(path) = matches.value_of("output") {
        if let Err(_) = check_output(path) {
            return;
        }
    }

    let simulator = toybox::get_simulation_by_name(game).unwrap();
    let (w, h) = simulator.game_size();
    let mut state = simulator.new_game();
    let mut images = VecDeque::with_capacity(max_frames.unwrap_or(num_steps));

    for _ in 0..num_steps {
        let mut buttons = Input::default();
        buttons.up = true;
        for _ in 0..frame_step {
            if (state.game_over()) {
                state = simulator.new_game();
            }
            state.update_mut(&buttons)
        }

        let mut img = ImageBuffer::alloc(w, h);
        render_to_buffer(&mut img, &state.draw());
        images.push_back(img);
        if let Some(mf) = max_frames {
            if images.len() > mf {
                let _ = images.pop_front();
            }
        }
    }

    if let Some(path) = matches.value_of("output") {
        // Check the folder again now in case of a data-race (best-effort).
        if let Err(_) = check_output(path) {
            return;
        }

        for (i, img) in images.into_iter().enumerate() {
            let file = File::create(Path::new(path).join(format!("amidar{:08}.png", i))).unwrap();
            let ref mut w = BufWriter::new(file);
            let mut encoder = png::Encoder::new(w, img.width as u32, img.height as u32);
            encoder.set(png::ColorType::RGBA).set(png::BitDepth::Eight);
            let mut writer = encoder.write_header().unwrap();
            writer.write_image_data(&img.data).unwrap();
        }
    }
}
