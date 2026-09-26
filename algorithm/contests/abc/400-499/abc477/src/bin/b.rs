#![allow(non_snake_case)]
use itertools::Itertools;
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
        n: usize, d: isize,
        x: [isize; n]
    }

    let mut res = vec![];
    for i in 0..n {
        let mut ok = true;
        for j in (0..n).filter(|&u| u != i) {
            if (x[i] - x[j]).abs() < d {
                ok = false
            }
        }

        if ok {
            res.push(i + 1);
        }
    }

    println!("{}", res.len());
    println!("{}", res.into_iter().join(" "));
}
