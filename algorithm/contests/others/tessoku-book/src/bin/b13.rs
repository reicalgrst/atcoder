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

    let mut res = 0;
    let mut sum = 0;
    let mut r = 0;

    for l in 0..n {
        while r < n && sum + a[r] <= k {
            sum += a[r];
            r += 1;
        }

        res += r - l;

        if l == r {
            r += 1;
        } else {
            sum -= a[l];
        }
    }

    println!("{res}");
}
