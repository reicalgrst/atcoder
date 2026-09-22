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
        n: usize, m: isize, s: isize,
    }

    let mut pos = s;
    let mut cnt = 0;

    for _ in 0..n {
        input! {
                c: char, a: isize
        }

        let first = match c {
            'R' => m - pos,
            'L' => pos,
            _ => unreachable!(),
        };

        if a <= first {
            match c {
                'R' => pos += a,
                'L' => pos -= a,
                _ => unreachable!(),
            }
            continue;
        }

        let a = a - first;
        cnt += 1 + (a - 1) / m;

        match (c, (a / m) % 2) {
            ('R', 0) | ('L', 1) => {
                pos = m - a % m;
            }
            ('R', 1) | ('L', 0) => {
                pos = a % m;
            }
            _ => unreachable!(),
        }
    }

    println!("{pos} {cnt}");
}
