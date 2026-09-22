#![allow(non_snake_case)]
use itertools::Itertools;
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
        n: usize,
        a: [isize; n]
    }

    if n == 1 {
        println!("0");
        return;
    }

    let mut res = 0;
    for b in a.into_iter().permutations(n) {
        let sum = b.windows(2).map(|v| (v[0] - v[1]).abs()).sum::<isize>();
        res = max(res, sum);
    }
    println!("{res}");
}
