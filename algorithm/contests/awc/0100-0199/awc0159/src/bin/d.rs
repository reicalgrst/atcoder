#![allow(non_snake_case)]
use proconio::{marker::Usize1, *};
use std::cmp::min;

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
        n: usize,
        v: [isize; n],
    }

    if n == 1 {
        println!("{}", v[0]);
        return;
    }

    input! {
        p: [Usize1; n - 1]
    }

    let mut sum = vec![0; n];
    for i in (1..n).rev() {
        sum[i] += v[i];
        sum[p[i - 1]] += sum[i];
    }

    let all = v.iter().sum::<isize>();
    let mut res = all;
    for i in 1..n {
        res = min(res, all - sum[i]);
    }
    println!("{res}");
}
