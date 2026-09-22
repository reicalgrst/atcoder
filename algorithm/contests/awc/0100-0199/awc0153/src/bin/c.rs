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
        n: usize, m: usize, k: isize,
        a: [isize; n],
        lr: [(usize, usize); m]
    }

    let mut imos = vec![0; n + 1];
    for (l, r) in lr {
        imos[l - 1] += 1;
        imos[r] -= 1;
    }

    for i in 0..n {
        imos[i + 1] += imos[i];
    }

    let mut res = 0;
    for i in 0..n {
        if imos[i] + a[i] >= k {
            res += 1;
        }
    }

    println!("{res}");
}
