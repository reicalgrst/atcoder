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
        n: usize, p: isize,
        xv: [(isize, isize); n]
    }

    let mut res = 0.0;
    for i in 0..n {
        let (x, v) = xv[i];
        if x == p {
            continue;
        }

        res += (v as f64) / ((x - p).abs() as f64);
    }

    println!("{res}");
}
