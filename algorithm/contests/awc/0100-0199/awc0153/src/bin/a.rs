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
        n: usize, k: usize,
        mut a: [isize; n],
        b: [Usize1; k],
        c: [isize; k]
    }

    for i in 0..k {
        a[b[i]] = c[i];
    }

    let res = a.windows(2).map(|v| (v[0] - v[1]).abs()).sum::<isize>();
    println!("{res}");
}
