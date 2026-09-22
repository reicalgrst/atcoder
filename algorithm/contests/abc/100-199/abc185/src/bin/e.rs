#![allow(non_snake_case)]
use itertools::iproduct;
use proconio::*;

// https://maguro.dev/blog/debug-macro/
#[allow(unused_macros)]
macro_rules! debug {
    ($($a:expr),* $(,)*) => {
        #[cfg(debug_assertions)]
        eprintln!(concat!($("| ", stringify!($a), " = {:?} "),*, "|"), $(&$a),*);
    };
}

// https://qiita.com/maguro_tuna/items/fab200fdc1efde1612e7
macro_rules! min {
    ($a:expr $(,)*) => {{
        $a
    }};
    ($a:expr, $b:expr $(,)*) => {{
        std::cmp::min($a, $b)
    }};
    ($a:expr, $($rest:expr),+ $(,)*) => {{
        std::cmp::min($a, min!($($rest),+))
    }};
}

macro_rules! chmin {
    ($base:expr, $($cmps:expr),+ $(,)*) => {{
        let cmp_min = min!($($cmps),+);
        if $base > cmp_min {
            $base = cmp_min;
            true
        } else {
            false
        }
    }};
}

const INF: usize = usize::MAX;

fn main() {
    input! {
        n: usize, m: usize,
        a: [usize; n],
        b: [usize; m]
    }

    // dp[i][j] := A[0..i] と B[0..j] での答え
    let mut dp = vec![vec![INF; m + 1]; n + 1];

    for i in 0..n + 1 {
        dp[i][0] = i;
    }

    for j in 0..m + 1 {
        dp[0][j] = j;
    }

    for (i, j) in iproduct!(0..n, 0..m) {
        let v1 = dp[i][j];
        let v2 = dp[i][j + 1];
        let v3 = dp[i + 1][j];

        if a[i] == b[j] {
            chmin!(dp[i + 1][j + 1], v1);
        } else {
            chmin!(dp[i + 1][j + 1], v1 + 1, v2 + 1, v3 + 1);
        }
    }

    println!("{}", dp[n][m]);
}
