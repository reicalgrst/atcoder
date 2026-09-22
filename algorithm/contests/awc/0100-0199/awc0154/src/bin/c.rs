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
        n: usize, k: isize, q: usize,
        c: [isize; n],
        lr: [(usize, usize); q]
    }

    let mut imos = vec![0; n + 1];
    for (l, r) in lr {
        imos[l - 1] += k;
        imos[r] -= k;
    }

    for i in 0..n {
        imos[i + 1] += imos[i];
    }

    let mut res = vec![];
    for i in 0..n {
        res.push(imos[i] + c[i]);
    }
    println!("{}", res.iter().join(" "));
}
