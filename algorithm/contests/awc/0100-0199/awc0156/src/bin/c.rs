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
        n: usize, k: usize,
        mut a: [isize; n]
    }

    a.sort();
    let mut diff = vec![];
    for i in 0..n - 1 {
        diff.push(a[i + 1] - a[i]);
    }
    diff.sort();

    let res = diff.into_iter().take(n - k).sum::<isize>();
    println!("{res}");
}
