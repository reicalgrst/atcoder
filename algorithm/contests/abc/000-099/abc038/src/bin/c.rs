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
        n: usize, a: [usize; n]
    }

    let mut res = 0;
    let mut r = 0;

    for l in 0..n {
        while r + 1 < n && a[r] < a[r + 1] {
            r += 1;
        }

        res += r - l + 1;

        if l == r {
            r += 1;
        }
    }

    println!("{res}");
}
