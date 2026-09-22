#![allow(non_snake_case)]
use proconio::{marker::Usize1, *};
use std::cmp::max;

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
        p: [Usize1; m]
    }

    let mut cnt = vec![0; n];
    let mut maxi = 0;
    let mut res = 0;

    for i in 0..m {
        cnt[p[i]] += 1;

        if p[i] == 1 {
            if cnt[p[i]] < maxi {
                res += 1;
            }
        }

        maxi = max(maxi, cnt[p[i]]);
    }

    println!("{res}");
}
