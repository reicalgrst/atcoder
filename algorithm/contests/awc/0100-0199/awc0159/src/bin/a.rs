#![allow(non_snake_case)]
use itertools::Itertools;
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
        n: usize, m: usize, k: usize,
        rc: [(Usize1, Usize1); k]
    }

    let mut s = vec![vec!['#'; m]; n];
    for (r, c) in rc {
        s[r][c] = '.';
    }

    println!("{}", s.into_iter().map(|v| v.iter().join("")).join("\n"));
}
