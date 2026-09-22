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
        n: usize, m: usize,
        mut a: [usize; n],
        mut b: [usize; m]
    }

    a.sort();
    b.sort();

    // 優先順位1
    let mut i = 0;
    for j in 0..m {
        while i < n && a[i] < b[j] {
            i += 1;
        }

        if i == n {
            println!("-1");
            return;
        }

        i += 1;
    }

    // 優先順位2
    let mut res = 0;
    let mut i = 0;
    let mut j = 0;

    while i < n && j < m {
        if a[i] == b[j] {
            res += 1;
            i += 1;
            j += 1;
        } else if a[i] > b[j] {
            j += 1;
        } else {
            i += 1;
        }
    }

    println!("{res}");
}
