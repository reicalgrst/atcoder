#![allow(non_snake_case)]
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
        n: usize, m: usize,
        mut s: [usize; n],
        tp: [(Usize1, usize); m]
    }

    for (t, p) in tp {
        if p <= s[t] {
            s[t] -= p;
        }
    }

    println!("{}", s.iter().filter(|&&i| i == 0).count());
}
