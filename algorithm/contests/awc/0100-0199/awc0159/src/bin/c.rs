#![allow(non_snake_case)]
use std::cmp::min;

use proconio::{marker::Usize1, *};

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
        _n: usize, m: usize, k: usize,
    }

    let mut masks = vec![0; m];
    for i in 0..m {
        input! {
            c: usize, s: [Usize1; c]
        }

        for &x in &s {
            masks[i] |= 1_usize << x;
        }
    }

    let mut res = usize::MAX;
    for bit in 0..(1_usize << m) {
        let mut now = 0;
        for i in 0..m {
            if bit & (1 << i) != 0 {
                now ^= masks[i];
            }
        }

        if now.count_ones() as usize == k {
            res = min(res, bit.count_ones() as usize);
        }
    }

    if res == usize::MAX {
        println!("-1");
    } else {
        println!("{res}");
    }
}
