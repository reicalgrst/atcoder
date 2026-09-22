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
        a: [usize; m]
    }

    let sum = a.iter().sum::<usize>();
    if sum <= n * k {
        println!("Yes");
    } else {
        println!("No");
    }
}
