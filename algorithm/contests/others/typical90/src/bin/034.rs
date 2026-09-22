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
        n: usize, k: usize,
        a: [usize; n]
    }

    let mut map = HashMap::new();
    let mut res = 0;
    let mut r = 0;

    for l in 0..n {
        while r < n && map.len() <= k {
            if map.len() == k && map.get(&a[r]).is_none() {
                break;
            }
            *map.entry(a[r]).or_insert(0) += 1;
            r += 1;
        }

        res = max(res, r - l);

        if l == r {
            r += 1;
        } else {
            let v = map.get_mut(&a[l]).unwrap();
            if *v == 1 {
                map.remove(&a[l]);
            } else {
                *v -= 1;
            }
        }
    }

    println!("{res}");
}
