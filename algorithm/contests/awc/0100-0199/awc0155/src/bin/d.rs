#![allow(non_snake_case)]
use proconio::*;
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
        n: usize, k: usize,
        a: [usize; n]
    }

    let mut csum = vec![0; n + 1];
    for i in 0..n {
        csum[i + 1] = csum[i] + a[i];
    }

    let m = n - k + 1;
    let mut v = vec![0; m];
    for i in 0..m {
        v[i] = csum[i + k] - csum[i];
    }

    debug!(csum, v);

    // maxi[i] := i + 1 日目以降の最大値
    let mut maxi = vec![0; m];
    let mut mx = 0;

    for i in (0..m).rev() {
        mx = max(mx, v[i]);
        maxi[i] = mx;
    }

    debug!(maxi);

    let mut res = 0;
    for i in 0..m {
        let j = i + k + 1;
        if m <= j {
            break;
        }

        res = max(res, v[i] + maxi[j]);
    }

    println!("{res}");
}
