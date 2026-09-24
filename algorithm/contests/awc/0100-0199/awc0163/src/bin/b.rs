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
        mut l: usize, w: usize,
    }

    let mut res = 0;
    while w < l {
        res += 1;
        l = (l + 1) / 2;
    }
    println!("{res}");
}
