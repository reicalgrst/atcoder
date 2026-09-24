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
        t: usize, n: usize, k: usize, m: usize,
        lr: [(usize, usize); n]
    }

    let max_right = t - k;

    // masks[i] := i に置いたときに被る施設の集合
    let mut masks = vec![0; max_right + 1];
    for i in 0..=max_right {
        for x in 0..n {
            let (l, r) = lr[x];
            if l < i + k && i < r {
                masks[i] |= 1_usize << x;
            }
        }
    }

    // dp[S] := 集合 S の状態になっているとき， 次に点検を開始しなければならない最大の位置
    let mut dp = vec![None; 1 << n];
    dp[0] = Some(m);

    for x in 0..max_right + 1 {
        for state in 0..1 << n {
            let Some(right) = dp[state] else {
                continue;
            };

            if right < x {
                continue;
            }

            let nstate = state | masks[x];
            let prev_right = dp[nstate].unwrap_or(0);
            let next_right = x + k + m;
            dp[nstate] = Some(max(prev_right, next_right));
        }
    }

    let mut res = n;
    for state in 0..1 << n {
        if dp[state].is_some_and(|right| t <= right) {
            res = res.min(state.count_ones() as usize);
        }
    }

    println!("{res}");
}
