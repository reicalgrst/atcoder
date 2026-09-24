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
        n: usize, m: usize, k: usize,
        a: [usize; n]
    }

    let need = a.into_iter().map(|u| (u + k - 1) / k).sum::<usize>();
    let res = need.saturating_sub(m);
    println!("{res}");
}
