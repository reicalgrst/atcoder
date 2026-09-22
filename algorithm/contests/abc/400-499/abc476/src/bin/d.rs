#![allow(non_snake_case)]
use proconio::*;
use std::cmp::max;
use superslice::Ext;

// https://maguro.dev/blog/debug-macro/
#[allow(unused_macros)]
macro_rules! debug {
    ($($a:expr),* $(,)*) => {
        #[cfg(debug_assertions)]
        eprintln!(concat!($("| ", stringify!($a), " = {:?} "),*, "|"), $(&$a),*);
    };
}

/// 自販機は K$ 紙幣しか受け入れないので先に見る
/// 残りのそれぞれについて， 累積和に二分探索すれば O(M log N) ？？？

fn main() {
    input! {
        n: usize, m: usize, k: u128,
        x: u128, y: u128,
        mut a: [u128; n],
        mut b: [u128; m]
    }

    a.sort();
    b.sort();

    let mut csum = vec![0; n + 1];
    for i in 0..n {
        csum[i + 1] = csum[i] + a[i];
    }

    let first_x = x + k * y;
    let first_idx = csum.upper_bound(&first_x) - 1;
    debug!(first_x, first_idx);

    let mut res = first_idx;

    let mut used_k = 0;
    let mut otsuri = 0;

    for i in 0..m {
        let use_k = (b[i] + k - 1) / k;
        used_k += use_k;
        otsuri += use_k * k - b[i];

        if y < used_k {
            break;
        }

        let now_x = x + otsuri + (y - used_k) * k;
        let idx = csum.upper_bound(&now_x) - 1;

        debug!(use_k, used_k, otsuri, now_x, idx);
        res = max(res, (i + 1) + idx);
    }

    println!("{}", res);
}
