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
        mut l: [usize; m]
    }

    a.sort();
    l.sort();

    let mut load = vec![];
    for i in 1..n {
        load.push(a[0] + a[i]);
    }

    let mut cnt = 0;
    for i in 0..m {
        if cnt < load.len() && load[cnt] <= l[i] {
            cnt += 1;
        }
    }

    if cnt == load.len() {
        println!("Yes");
    } else {
        println!("No");
    }
}
