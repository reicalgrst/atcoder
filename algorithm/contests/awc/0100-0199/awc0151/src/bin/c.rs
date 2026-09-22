#![allow(non_snake_case)]
use itertools::Itertools;
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
        n: usize, q: usize,
        at: [(isize, isize); n],
    }

    let a = at.iter().map(|&(u, _)| u).collect_vec();
    let t = at.iter().map(|&(_, v)| v).collect_vec();

    let mut imos = vec![0; n + 1];

    for _ in 0..q {
        input! {
            l: usize, r: usize, x: isize
        }

        imos[l - 1] += x;
        imos[r] -= x;
    }

    for i in 0..n {
        imos[i + 1] += imos[i];
    }

    let mut res = 0;
    for i in 0..n {
        if imos[i] + a[i] >= t[i] {
            res += 1;
        }
    }

    println!("{res}");
}
