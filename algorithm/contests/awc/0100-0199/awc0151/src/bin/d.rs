#![allow(non_snake_case)]
use itertools::iproduct;
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
        N: usize, M: usize, K: usize, B: usize,
        dvt: [(usize, usize, usize); N]
    }

    // dp[i][j] := i 日目に j 個の仕事を完了したときの MAX 報酬
    let mut dp = vec![vec![0; N + 1]; M + 1];

    for i in 0..N {
        let (d, v, t) = dvt[i];

        if t < d {
            continue;
        }

        for (j, k) in iproduct!(0..=i, (0..=t - d).rev()) {
            dp[k + d][j + 1] = max(dp[k + d][j + 1], dp[k][j] + v);
        }
    }

    let mut res = 0;
    for (i, j) in iproduct!(0..=M, 0..=N) {
        let v = dp[i][j] + if j >= K { B } else { 0 };
        res = max(res, v);
    }

    println!("{res}");
}
