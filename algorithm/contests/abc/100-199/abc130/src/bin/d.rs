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
        n: usize, k: usize,
        a: [usize; n]
    }

    let mut res = n * (n + 1) / 2;
    let mut sum = 0;
    let mut r = 0;

    for l in 0..n {
        while r < n && sum < k {
            sum += a[r];
            r += 1;
        }

        res -= r - l - 1;
        if r == n && sum < k {
            res -= 1;
        }

        sum -= a[l];
    }

    println!("{res}");
}
