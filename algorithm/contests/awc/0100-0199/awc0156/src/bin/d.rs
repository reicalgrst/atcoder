#![allow(non_snake_case)]
use bitset_fixed::BitSet;
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
        n: usize, a: [usize; n]
    }

    let sum = a.iter().sum::<usize>();
    let mut dp = BitSet::new(sum + 1);
    dp.set(0, true);

    for i in 0..n {
        dp |= &(&dp << a[i]);
    }

    let mut res = 0;
    for i in 0..=sum {
        if dp[i] {
            res = max(res, min(i, sum - i));
        }
    }
    println!("{res}");
}
