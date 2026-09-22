#![allow(non_snake_case)]
use proconio::{marker::Usize1, *};

// https://maguro.dev/blog/debug-macro/
#[allow(unused_macros)]
macro_rules! debug {
    ($($a:expr),* $(,)*) => {
        #[cfg(debug_assertions)]
        eprintln!(concat!($("| ", stringify!($a), " = {:?} "),*, "|"), $(&$a),*);
    };
}

fn dist(p1: (usize, usize), p2: (usize, usize)) -> isize {
    let dx = p1.0 as isize - p2.0 as isize;
    let dy = p1.1 as isize - p2.1 as isize;
    (dx * dx + dy * dy).isqrt() as isize
}

fn main() {
    input! {
        n: usize, q: usize,
        xy: [(usize, usize); n]
    }

    let mut diff = vec![0; n];
    for i in 0..n {
        let p1 = xy[i];
        for j in 0..n {
            let p2 = xy[j];
            diff[i] += dist(p1, p2);
        }
    }

    for _ in 0..q {
        input! {
            c: Usize1
        }

        println!("{}", diff[c]);
    }
}
