#![allow(non_snake_case)]
use proconio::*;
use std::cmp::min;
use superslice::Ext;

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
        x: [usize; n],
        p: [usize; m]
    }

    for i in 0..n {
        let idx = p.lower_bound(&x[i]);

        let res = if idx == 0 {
            p[0] - x[i]
        } else if idx == m {
            x[i] - p[m - 1]
        } else {
            min(p[idx] - x[i], x[i] - p[idx - 1])
        };

        println!("{res}");
    }
}
