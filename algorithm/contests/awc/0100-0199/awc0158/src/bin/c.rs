#![allow(non_snake_case)]
use itertools::Itertools;
use proconio::*;
use superslice::Ext;

// https://maguro.dev/blog/debug-macro/
#[allow(unused_macros)]
macro_rules! debug {
    ($($a:expr),* $(,)*) => {
        #[cfg(debug_assertions)]
        eprintln!(concat!($("| ", stringify!($a), " = {:?} "),*, "|"), $(&$a),*);
    };
}

fn main() {
    input! {
        n: usize, v: isize, q: usize,
        tw: [(isize, isize); n]
    }

    let t = tw.iter().map(|&(t, _)| t).collect_vec();
    let additional = |sp: isize| -> isize {
        if sp > 0 {
            v
        } else if sp < 0 {
            -v
        } else {
            0
        }
    };

    let mut speed = vec![0; n + 1];
    for i in 0..n {
        let (_, w) = tw[i];
        speed[i + 1] = w;
    }
    debug!(speed);

    let mut pos = vec![0; n + 1];
    for i in 1..n {
        let pt = t[i - 1];
        pos[i + 1] = pos[i] + (speed[i] + additional(speed[i])) * (t[i] - pt);
    }
    debug!(pos);

    for _ in 0..q {
        input! {
            x: isize,
        }

        let idx = t.lower_bound(&x);
        let res = if idx == 0 {
            0
        } else {
            pos[idx] + (x - t[idx - 1]) * (speed[idx] + additional(speed[idx]))
        };

        println!("{res}");
    }
}
