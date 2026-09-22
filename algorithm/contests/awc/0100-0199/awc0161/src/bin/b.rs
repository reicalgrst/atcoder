#![allow(non_snake_case)]
use proconio::*;
use std::cmp::Reverse;

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
        n: usize, mut a: [usize; n]
    }

    a.sort_by_key(|&u| Reverse(u));
    let res = a.chunks(2).map(|v| v[0]).sum::<usize>();
    println!("{res}");
}
