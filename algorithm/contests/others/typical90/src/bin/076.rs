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
        n: usize,
        a: [usize; n]
    }

    let total = a.iter().sum::<usize>();
    if total % 10 != 0 {
        println!("No");
        return;
    }

    let mut sum = 0;
    let mut r = 0;
    for l in 0..2 * n {
        while r < 2 * n && sum + a[r % n] <= total / 10 {
            sum += a[r % n];
            r += 1;
        }

        if sum == total / 10 {
            println!("Yes");
            return;
        }

        if l == r {
            r += 1;
        } else {
            sum -= a[l % n];
        }
    }

    println!("No");
}
