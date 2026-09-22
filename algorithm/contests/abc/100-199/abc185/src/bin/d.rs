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
        n: usize, m: usize,
        mut a: [usize; m]
    }

    if m == 0 {
        println!("1");
        return;
    }

    a.sort();
    let mut diff = vec![];
    for i in 0..m - 1 {
        diff.push(a[i + 1] - a[i] - 1);
    }

    if a[0] != 1 {
        diff.push(a[0] - 1);
    }

    if a[m - 1] != n {
        diff.push(n - a[m - 1]);
    }

    let diff = diff.into_iter().filter(|&u| u != 0).collect_vec();
    if diff.len() == 0 {
        println!("0");
        return;
    }

    debug!(diff);

    let min = *diff.iter().min().unwrap();
    let mut res = 0;
    for d in diff {
        res += (d + min - 1) / min;
    }
    println!("{res}");
}
