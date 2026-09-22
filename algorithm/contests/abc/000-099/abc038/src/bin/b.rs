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
        h1: usize, w1: usize,
        h2: usize, w2: usize
    }

    let res = if h1 == h2 || h1 == w2 || w1 == h2 || w1 == w2 {
        "YES"
    } else {
        "NO"
    };

    println!("{res}");
}
