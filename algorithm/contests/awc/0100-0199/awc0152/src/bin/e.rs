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

fn main() {
    input! {
        n: usize, m: usize,
        uv: [(Usize1, Usize1); m],
        q: usize
    }

    for _ in 0..q {
        solve(n, m, &uv);
    }
}

fn solve(n: usize, m: usize, uv: &Vec<(usize, usize)>) {}
