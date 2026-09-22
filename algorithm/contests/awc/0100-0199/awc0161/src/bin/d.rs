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
        n: usize,
        a: [isize; n]
    }

    if n == 1 {
        println!("{}", a[0]);
        return;
    }

    // dp[l][r][p] := [l, r) でゲームしたときの最大値
    let mut dp = vec![vec![isize::MIN; n + 1]; n + 1];
    for i in 0..n {
        dp[i][i + 1] = a[i];
    }

    for width in 2..=n {
        for l in 0..n {
            let r = l + width;
            if n < r {
                continue;
            }

            if n % 2 == width % 2 {
                dp[l][r] = max(dp[l][r - 1], dp[l + 1][r]);
            } else {
                dp[l][r] = min(dp[l][r - 1], dp[l + 1][r]);
            }
        }
    }

    println!("{}", dp[0][n]);
}
