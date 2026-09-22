#![allow(non_snake_case, dead_code)]
mod beam;
mod greedy;
mod problem_io;

use ahc_library::random::*;
use beam::*;
use problem_io::*;

pub fn main() {
    let input = Input::new();
    let mut rng = Random::new(123456789);
    beam_search(&input, &mut rng, 1500);

    /*
    let timer = TimeKeeper::new(2.0);
    let mut rng = Random::new(123456789);

    let mut current_state = greedy(&input);
    let mut best_state = current_state.clone();

    // tuned with rustuna
    let t0: f64 = 85.89;
    let t1: f64 = 3.07;

    while !timer.is_over() {
        let y = rng.usize(0, input.H);
        if let Ok(new_state) = match rng.usize(0, 2) {
            0 => width_randomizer(&input, &current_state, &mut rng, y),
            1 => shift_randomizer(&input, &current_state, &mut rng, y),
            _ => unreachable!(),
        } {
            // 上の方を変えるインセンティブはあるけど， 下の方はあんまりない
            let weight = (y as f64) / (input.H as f64 - 1.0);
            let progress = timer.progress() * weight;

            let temp = t0.powf(1.0 - progress) * t1.powf(progress);

            let delta = current_state.score - new_state.score;
            if 0 <= delta || rng.f64() < (delta as f64 / temp).exp() {
                current_state = new_state;
            }

            if current_state.score < best_state.score {
                best_state = current_state.clone();
            }
        }
    }

    best_state.output();
    */
}
