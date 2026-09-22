#![allow(non_snake_case)]
use proconio::*;
use std::cmp::{max, min};

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
        n: usize, L: usize, R: usize,
        t: [usize; n]
    }

    let mut sum = 0;
    let mut min_sum = 0;
    let mut res = isize::MIN;

    for i in 0..n {
        let v = if L <= t[i] && t[i] <= R { 1 } else { -1 };
        sum += v;
        res = max(res, sum - min_sum);
        min_sum = min(sum, min_sum);
    }

    println!("{res}");
}
