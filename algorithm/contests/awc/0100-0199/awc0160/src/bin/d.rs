#![allow(non_snake_case)]
use proconio::{marker::Usize1, *};
use std::collections::BTreeMap;

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
        n: usize, q: usize,
        a: [usize; n]
    }

    let mut map = BTreeMap::new();
    for &x in &a {
        *map.entry(x).or_insert(0) += 1;
    }

    for _ in 0..q {
        input! {
            t: Usize1, x: usize, r: usize
        }

        if r == 1 {
            continue;
        }

        let (k, v) = if t == 0 {
            map.pop_last().unwrap()
        } else {
            map.pop_first().unwrap()
        };

        if v != 1 {
            map.insert(k, v - 1);
        }

        *map.entry(x).or_insert(0) += 1;
    }

    let mut res = 0;
    for (k, v) in map {
        res += k * v;
    }
    println!("{res}");
}
