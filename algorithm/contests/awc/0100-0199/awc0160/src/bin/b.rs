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
        n: usize, a: [isize; n]
    }

    let mut res = isize::MIN;
    let mut min_sum = 0;
    let mut sum = 0;

    for i in 0..n {
        sum += a[i] - 1;
        res = res.max(sum - min_sum);
        min_sum = min_sum.min(sum);
    }

    println!("{res}");
}
