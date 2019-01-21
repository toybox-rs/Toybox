extern crate toybox;
extern crate toybox_core;

use toybox::graphics::ImageBuffer;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let backup: String = "1000".to_string();
    let n_steps = args.get(2).unwrap_or(&backup);
    println!("n_steps: {}", n_steps);
    let n_steps = n_steps.parse::<usize>().unwrap();
    let game = &args[1];
    println!("{} for {}.", game, n_steps);
    let mut sim = toybox::get_simulation_by_name(game).unwrap();
    let mut state = sim.new_game();
    let actions = sim.legal_action_set();
    println!("actions: {:?}", actions);
    let mut scores = Vec::new();
    let mut byte_sum: usize = 0;
    for i in 0..n_steps {
        let action = actions[i % actions.len()];
        state.update_mut(action.to_input());
        if state.lives() <= 0 {
            scores.push(state.score());
            state = sim.new_game();
        }
        let (w, h) = sim.game_size();
        let mut img = ImageBuffer::alloc(w, h);
        img.render(&state.draw());

        // use the images so rust doesn't optimize it out.
        byte_sum += img.data[0] as usize;
    }
    scores.push(state.score());
    let mut total = 0.0;
    for s in scores.iter() {
        total += *s as f64;
    }
    println!("Average Episode Score: {}", total / scores.len() as f64);
    println!("Byte Sum: {}", byte_sum);
}
