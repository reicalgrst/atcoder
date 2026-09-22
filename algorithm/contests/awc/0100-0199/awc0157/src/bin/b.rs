#![allow(non_snake_case)]
use proconio::*;
use std::cmp::min;

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
        n: usize,
        ab: [(isize, isize); n]
    }

    let mut mini = 0;
    let mut sum = 0;
    for (a, b) in ab {
        sum += a - b;
        mini = min(mini, sum);
    }

    let res = if mini < 0 { -mini } else { 0 };
    println!("{res}");
}
