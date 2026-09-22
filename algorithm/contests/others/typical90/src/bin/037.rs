#![allow(non_snake_case)]
use ac_library::segtree::{Max, Segtree};
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
        w: usize, n: usize,
        lrv: [(isize, isize, usize); n]
    }

    let mut dp = vec![isize::MIN; w + 1];
    dp[0] = 0;

    for i in 0..n {
        let segtree = Segtree::<Max<isize>>::from_iter(dp.clone());
        let mut ndp = dp.clone();
        let (l, r, v) = lrv[i];

        for pos in 0..=w as isize {
            if pos - l < 0 {
                continue;
            }

            let left = 0.max(pos - r) as usize;
            let right = w.min((pos - l) as usize);

            let mv = segtree.prod(left..=right);
            if mv < 0 {
                continue;
            }
            chmax!(ndp[pos as usize], mv + v as isize);
        }

        dp = ndp.clone();
    }

    if dp[w] == isize::MIN {
        println!("-1");
    } else {
        println!("{}", dp[w]);
    }
}
