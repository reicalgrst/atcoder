#![allow(non_snake_case)]
use proconio::*;
use std::cmp::max;
use std::collections::HashMap;

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

    let mut map = HashMap::new();
    let mut res = 0;
    let mut r = 0;

    for l in 0..n {
        while r < n && map.get(&a[r]).is_none() {
            map.insert(a[r], 1);
            r += 1;
        }

        res = max(res, r - l);

        if r == l {
            r += 1;
        } else {
            map.remove(&a[l]);
        }
    }

    println!("{res}");
}
