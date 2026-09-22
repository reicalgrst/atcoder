#![allow(non_snake_case)]
use ac_library::{Segtree, segtree::Min};
use proconio::*;
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
        n: usize, l: usize, r: usize,
        x: [usize; n]
    }

    let mut dp = Segtree::<Min<usize>>::from_iter(std::iter::repeat(usize::MAX).take(n));
    dp.set(0, 0);

    for i in 1..n {
        let pos = x[i];

        // 解の存在は保証されている
        let left = pos.saturating_sub(r);
        let right = pos - l;

        let left_idx = x.lower_bound(&left);
        let right_idx = x.upper_bound(&right);

        let mini = dp.prod(left_idx..right_idx);
        if mini != usize::MAX {
            dp.set(i, mini + 1);
        }
    }

    println!("{}", dp.get(n - 1));
}
