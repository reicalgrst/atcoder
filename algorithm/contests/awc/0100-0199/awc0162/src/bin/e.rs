#![allow(non_snake_case)]
use itertools::iproduct;
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

const INF: usize = usize::MAX;

fn main() {
    input! {
        n: usize, m: usize, t: u128,
        s: [u128; n],
        uvw: [(usize, usize, isize); m]
    }

    let mut graph = vec![vec![-1; n]; n];
    for (u, v, w) in uvw {
        graph[u][v] = w;
    }

    debug!(n, m, t);
    debug!(s);
    debug!(graph);

    let mut dp = vec![vec![INF; n]; 1 << n];
    dp[1][0] = 0;

    let mut res = INF;
    for bit in 1_usize..(1 << n) {
        if bit & 1 == 0 {
            continue;
        }

        for (u, v) in iproduct!(0..n - 1, 1..n) {
            if bit & (1 << u) == 0 || bit & (1 << v) == 0 {
                continue;
            }

            let prev = bit & !(1 << v);
            let cost = graph[u][v];

            if cost == -1 || dp[prev][u] == INF {
                continue;
            }

            let cost = cost as usize;
            dp[bit][v] = min(dp[bit][v], dp[prev][u] + cost);
        }

        // 更新
        if bit & (1 << (n - 1)) != 0 {
            let mut sum = 1;
            for i in 0..n {
                if bit & (1 << i) != 0 {
                    sum *= s[i];
                }
            }

            if t <= sum {
                res = min(res, dp[bit][n - 1]);
            }
        }
    }

    if res == INF {
        println!("-1");
    } else {
        println!("{res}");
    }
}
