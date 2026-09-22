#![allow(non_snake_case)]
use proconio::{marker::Usize1, *};

// https://maguro.dev/blog/debug-macro/
#[allow(unused_macros)]
macro_rules! debug {
    ($($a:expr),* $(,)*) => {
        #[cfg(debug_assertions)]
        eprintln!(concat!($("| ", stringify!($a), " = {:?} "),*, "|"), $(&$a),*);
    };
}

// https://qiita.com/maguro_tuna/items/fab200fdc1efde1612e7
macro_rules! max {
    ($a:expr $(,)*) => {{
        $a
    }};
    ($a:expr, $b:expr $(,)*) => {{
        std::cmp::max($a, $b)
    }};
    ($a:expr, $($rest:expr),+ $(,)*) => {{
        std::cmp::max($a, max!($($rest),+))
    }};
}

macro_rules! chmax {
    ($base:expr, $($cmps:expr),+ $(,)*) => {{
        let cmp_max = max!($($cmps),+);
        if $base < cmp_max {
            $base = cmp_max;
            true
        } else {
            false
        }
    }};
}

fn main() {
    input! {
        n: usize, m: usize, k: isize,
        h: [isize; n],
        pr: [(Usize1, isize); m]
    }

    let mut rv = vec![0; n];
    for (p, r) in pr {
        rv[p] += r;
    }

    let mut dp = vec![-1; n];
    dp[0] = k - h[0];

    if dp[0] < 0 {
        println!("-1");
        return;
    }

    dp[0] += rv[0];

    for i in 0..n {
        if i + 1 < n {
            if dp[i] - h[i + 1] >= 0 {
                chmax!(dp[i + 1], dp[i] - h[i + 1] + rv[i + 1]);
            }
        }

        if i + 2 < n {
            if dp[i] - h[i + 2] >= 0 {
                chmax!(dp[i + 2], dp[i] - h[i + 2] + rv[i + 2]);
            }
        }
    }

    println!("{}", dp[n - 1]);
}
