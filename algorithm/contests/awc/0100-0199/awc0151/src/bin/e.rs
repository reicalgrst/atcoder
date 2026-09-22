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
        n: usize
    }

    if n == 1 {
        println!("1");
        return;
    }

    let mut is_prime = vec![true; n + 1];
    is_prime[0] = false;
    is_prime[1] = false;

    let mut p = 2;
    while p * p <= n {
        let mut x = p * 2;

        while x <= n {
            is_prime[x] = false;
            x += p;
        }

        p += 1;
    }

    let res = is_prime.iter().filter(|&&u| u).count();
    println!("{res}");
}
