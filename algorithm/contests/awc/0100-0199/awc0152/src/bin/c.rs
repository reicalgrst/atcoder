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

const MOD: usize = 998244353;

fn main() {
    input! {
        n: usize, k: usize,
        a: [usize; n]
    }

    // dp[i] := 余りが i である場合の数
    let mut dp = vec![0; k];
    dp[0] = 1;

    for i in 0..n {
        let mut ndp = dp.clone();

        for j in 0..k {
            ndp[(j + a[i]) % k] = (dp[j] + ndp[(j + a[i]) % k]) % MOD;
        }

        dp = ndp;
    }

    println!("{}", dp[0] - 1);
}
