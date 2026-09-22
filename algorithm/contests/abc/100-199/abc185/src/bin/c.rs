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
        l: usize
    }

    let mut res = 1;
    let mut divs = vec![false; 11 + 1];

    for i in (l - 11)..=(l - 1) {
        res *= i;
        for j in 1..=11 {
            if divs[j] {
                continue;
            }

            if res % j == 0 {
                res /= j;
                divs[j] = true;
            }
        }
    }

    println!("{res}");
}
