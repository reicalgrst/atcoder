#![allow(non_snake_case)]
use proconio::*;
use std::cmp::max;

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
        n: usize, s: isize, c: isize,
        r: [isize; n]
    }

    let sum = r.into_iter().sum::<isize>();
    let res = max(sum - s, 0) * c;
    println!("{res}");
}
