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

const INF: isize = isize::MAX;

fn main() {
    input! {
        n: usize,
        a: [isize; n]
    }

    let mut dp = vec![INF; n + 1];
    dp[0] = 0;
    dp[1] = 0;

    for i in 1..n {
        if i + 1 <= n {
            dp[i + 1] = min(dp[i + 1], dp[i] + max(a[i] - a[i - 1], 0));
        }

        if i + 2 <= n {
            dp[i + 2] = min(dp[i + 2], dp[i] + max(a[i + 1] - a[i - 1], 0));
        }
    }

    println!("{}", dp[n]);
}
