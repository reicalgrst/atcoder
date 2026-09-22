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
        n: usize, x: isize,
        mut ab: [(isize, isize); n]
    }

    ab.sort_by(|(h1, v1), (h2, v2)| h2.cmp(&h1).then(v2.cmp(&v1)));

    let mut res = 0;
    let mut prev = x;

    for (h, v) in ab {
        if h < prev && 0 < v {
            prev = h;
            res += v;
        }
    }

    println!("{res}");
}
