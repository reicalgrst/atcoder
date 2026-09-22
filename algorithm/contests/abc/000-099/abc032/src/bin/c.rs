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
        n: usize, k: u128,
        s: [u128; n]
    }

    if s.iter().contains(&0) {
        println!("{}", s.len());
        return;
    }

    let mut res = 0;
    let mut prod = 1;
    let mut r = 0;

    for l in 0..n {
        while r < n && prod * s[r] <= k {
            prod *= s[r];
            r += 1;
        }

        res = max(res, r - l);
        if l == r {
            r += 1;
        } else {
            prod /= s[l];
        }
    }

    println!("{res}");
}
