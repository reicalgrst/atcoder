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
        n: usize, m: usize,
        mut s: [usize; n],
        mut d: [usize; m]
    }

    s.sort();
    d.sort();

    let mut j = 0;
    for i in 0..n {
        if j < m && s[i] >= d[j] {
            j += 1;
        }
    }

    println!("{j}");
}
