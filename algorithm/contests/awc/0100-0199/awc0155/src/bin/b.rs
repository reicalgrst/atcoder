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
        n: usize, d: usize, s: Usize1,
        a: [usize; n]
    }

    let mut res = 0;
    let sum = a.iter().sum::<usize>();
    let first = n - s;

    if d <= first {
        for i in s..s + d {
            res += a[i];
        }
        println!("{res}");
        return;
    }

    for i in s..n {
        res += a[i];
    }

    let rem = d - first;
    res += (rem / n) * sum;

    let rem = rem % n;
    for i in 0..rem {
        res += a[i];
    }

    println!("{res}");
}
