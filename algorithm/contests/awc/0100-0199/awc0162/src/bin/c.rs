#![allow(non_snake_case)]
use ac_library::segtree::{Min, Segtree};
use proconio::*;
use std::cmp::min;
use superslice::Ext;

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
        n: usize, d: usize,
        mut ac: [(usize, usize); n]
    }

    ac.sort_by_key(|&(a, _)| a);
    let mut cost_tree = Segtree::<Min<usize>>::new(n);
    for i in 0..n {
        let (_, c) = ac[i];
        cost_tree.set(i, c);
    }

    let mut res = usize::MAX;
    for i in 0..n {
        let (a, c) = ac[i];
        if d <= a {
            res = min(res, c);
        }

        let rem = d.saturating_sub(a);
        if rem == 0 {
            continue;
        }

        let idx = ac.lower_bound_by_key(&rem, |&(a2, _)| a2);
        let left = idx.max(i + 1);

        if left < n {
            let mini = cost_tree.prod(left..n);
            res = min(res, c + mini);
        }
    }

    if res == usize::MAX {
        println!("-1");
    } else {
        println!("{res}");
    }
}
