use crate::problem_io::*;
use ahc_library::random::Random;
use bitset_fixed::BitSet;
use std::cmp::Reverse;

#[derive(Clone)]
pub struct BeamState {
    current_mask: usize,
    current_placed: BitSet,
    next_mask: usize,
    x: usize,
    penalty: isize,
    score: isize,
    parent: Option<usize>,
}

fn get_bit(input: &Input, x: usize, width: usize) -> usize {
    (width / 2) * input.W + x
}

enum Actions {
    Skip,
    Place(usize),
}

impl Actions {
    fn all_actions() -> Vec<Actions> {
        let mut res = vec![Actions::Skip];
        for i in 0..5 {
            res.push(Actions::Place(2 * i + 1));
        }
        res
    }
}

impl BeamState {
    fn initial(input: &Input) -> Self {
        let initial_score = (5 * input.H * input.W + 1) as isize;
        Self {
            current_mask: 0,
            current_placed: BitSet::new(5 * input.W + 1),
            next_mask: 0,
            x: 0,
            penalty: 0,
            score: initial_score,
            parent: None,
        }
    }

    fn start_next_row(input: &Input, prev: &BeamState, parent: usize) -> Self {
        Self {
            current_mask: prev.next_mask,
            current_placed: BitSet::new(5 * input.W + 1),
            next_mask: 0,
            x: 0,
            penalty: 0,
            score: prev.score,
            parent: Some(parent),
        }
    }

    fn legal_actions(&self, input: &Input, y: usize) -> Vec<Actions> {
        let mut res = vec![];
        let mask = self.current_mask | input.where_to_fill[y];

        for next_action in Actions::all_actions() {
            match next_action {
                Actions::Skip => {
                    if mask & (1_usize << self.x) == 0 {
                        res.push(Actions::Skip);
                    }
                }

                Actions::Place(width) => {
                    if input.W < self.x + width {
                        continue;
                    }

                    let width_mask = ((1_usize << width) - 1) << self.x;
                    if mask & width_mask == 0 {
                        continue;
                    }

                    res.push(Actions::Place(width));
                }
            }
        }

        res
    }

    fn apply_action(&mut self, input: &Input, action: Actions, y: usize) {
        if let Actions::Place(width) = action {
            self.next_mask |= 1_usize << (self.x + width / 2);
            self.current_placed
                .set(get_bit(&input, self.x, width), true);
            self.x += width;
            self.score -= input.c[width / 2];
            self.evaluate(input, y);
        } else {
            self.x += 1;
        }
    }

    // なるべく 1 と 0 を固める
    fn evaluate(&mut self, input: &Input, y: usize) {
        if 0 < y {
            let mask = self.current_mask | input.where_to_fill[y - 1];
            let change = (mask ^ (mask >> 1)).count_ones() as isize;
            self.penalty = 5 * (self.next_mask | input.where_to_fill[y - 1]).count_ones() as isize
                + 3 * change;
        }
    }
}

// 上位 2 / 5 + random 3 / 5
// ここを焼きなましっぽくする
fn prune(v: &mut Vec<BeamState>, rng: &mut Random, input: &Input, beam_width: usize, y: usize) {
    if v.len() <= beam_width {
        return;
    }

    v.sort_unstable_by_key(|s| Reverse((s.score - s.penalty, s.score)));
    let good = ((beam_width * 2 + 4) / 5).min(beam_width);
    let remain = beam_width - good;

    let progress = ((input.H - y) as f64) / (input.H as f64 - 1.0);
    let extra_good = (remain as f64 * progress) as usize;

    let keep = (good + extra_good).min(v.len());
    let mut rest = v.split_off(keep);

    while v.len() < beam_width && !rest.is_empty() {
        let i = rng.usize(0, rest.len());
        v.push(rest.swap_remove(i));
    }
}

pub fn beam_search(input: &Input, rng: &mut Random, beam_width: usize) {
    let mut history = vec![vec![]; input.H + 1];
    history[input.H] = vec![BeamState::initial(input)];

    for y in (0..input.H).rev() {
        let mut bucket = vec![vec![]; input.W + 1];
        bucket[0] = history[y + 1]
            .iter()
            .enumerate()
            .map(|(i, prev)| BeamState::start_next_row(input, prev, i))
            .collect();

        for x in 0..input.W {
            prune(&mut bucket[x], rng, input, beam_width, y);
            let states = std::mem::take(&mut bucket[x]);
            for state in states {
                for action in state.legal_actions(input, y) {
                    let mut next = state.clone();
                    next.apply_action(input, action, y);
                    bucket[next.x].push(next);
                }
            }
        }

        prune(&mut bucket[input.W], rng, input, beam_width, y);
        history[y] = bucket[input.W].clone();
    }

    let (mut state_idx, _) = history[0]
        .iter()
        .enumerate()
        .max_by_key(|(_, s)| s.score)
        .unwrap();

    let mut res = vec![];
    for y in 0..input.H {
        let state = &history[y][state_idx];
        for k in 0..5 {
            let width = 2 * k + 1;
            for x in 0..=input.W - width {
                let bit = get_bit(input, x, width);
                if state.current_placed[bit] {
                    res.push((x, y, width));
                }
            }
        }
        state_idx = state.parent.unwrap();
    }

    println!("{}", res.len());
    for &(x, y, width) in &res {
        println!("{} {} {}", x, y, width);
    }
}
