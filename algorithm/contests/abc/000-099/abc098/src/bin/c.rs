#![allow(non_snake_case)]
use proconio::{marker::Chars, *};
use std::cmp::min;

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
        s: Chars
    }

    let mut csum_e = vec![0; n + 1];
    let mut csum_w = vec![0; n + 1];

    for i in 0..n {
        csum_e[i + 1] = csum_e[i] + (s[i] == 'E') as usize;
        csum_w[i + 1] = csum_w[i] + (s[i] == 'W') as usize;
    }

    let mut res = usize::MAX;
    for i in 0..n {
        let l = csum_w[i];
        let r = csum_e[n] - csum_e[i + 1];
        res = min(res, l + r);
    }

    println!("{res}");
}
