#![allow(non_snake_case)]
use proconio::*;

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
        mut wh: [(usize, usize); n]
    }

    wh.sort_by(|&(w1, h1), &(w2, h2)| w1.cmp(&w2).then(h2.cmp(&h1)));
    debug!(wh);

    // dp[i] := i 個目の箱が外側にあるとき，　最大の個数
    let mut dp = vec![0; n + 1];
    dp[0] = 0;
}
