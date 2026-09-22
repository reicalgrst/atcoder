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
        n: isize, m: usize, t: isize,
        ab: [(isize, isize); m]
    }

    let mut battery = n;
    let mut prev_leave = 0;
    for (a, b) in ab {
        battery -= a - prev_leave;
        if battery <= 0 {
            println!("No");
            return;
        }
        battery = n.min(battery + b - a);
        prev_leave = b;
    }

    battery -= t - prev_leave;
    let res = if battery > 0 { "Yes" } else { "No" };
    println!("{res}");
}
