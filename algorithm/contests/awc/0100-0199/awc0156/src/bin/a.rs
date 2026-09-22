#![allow(non_snake_case)]
use itertools::Itertools;
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
        n: usize, m: usize,
        mut a: [isize; n],
        bc: [(Usize1, isize); m]
    }

    for (b, c) in bc {
        if c <= a[b] {
            a[b] -= c;
        }
    }

    println!("{}", a.iter().join(" "));
}
