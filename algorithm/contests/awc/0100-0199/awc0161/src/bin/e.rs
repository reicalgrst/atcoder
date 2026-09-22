#![allow(non_snake_case)]
use ac_library::segtree::{Max, Segtree};
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
        n: usize, m: usize,
        h: [usize; n],
        w: [usize; m]
    }

    let mut csum = vec![0; n + 1];
    for i in 0..n {
        csum[i + 1] = csum[i] + h[i];
    }
    let segtree = Segtree::<Max<usize>>::from_iter(h);

    let w = w.into_iter().max().unwrap();
    let mut res = 0;

    for i in 0..=n - w {
        let right = i + w;
        let mh = segtree.prod(i..right);
        res = max(res, mh * w + csum[i] + csum[n] - csum[right]);
    }
    println!("{res}");
}
