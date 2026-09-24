#![allow(non_snake_case)]
use proconio::{marker::Usize1, *};

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
        uvw: [(Usize1, Usize1, usize); n-1]
    }

    let res = uvw.into_iter().map(|(_, _, w)| w).sum::<usize>();
    println!("{res}");
}
