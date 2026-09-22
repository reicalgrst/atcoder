#![allow(non_snake_case)]
use proconio::{
    marker::{Chars, Usize1},
    *,
};

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
        n: usize, q: usize, m: usize,
        a: [usize; n],
        mut s: Chars,
    }

    for _ in 0..m {
        input! {
            t: usize
        }

        match t {
            1 => {
                input! {
                    p: Usize1, c: char
                }

                s[p] = c;
            }

            2 => {
                let mut res = 0;
                let mut pos = 0;
                let mut used = vec![false; n];

                for i in 0..q {
                    match s[i] {
                        'L' => {
                            if 0 < pos {
                                pos -= 1;
                            }
                        }
                        'R' => {
                            if pos < n - 1 {
                                pos += 1;
                            }
                        }
                        'P' => {
                            if used[pos] {
                                continue;
                            }
                            res += a[pos];
                            used[pos] = true;
                        }
                        'B' => {
                            pos = 0;
                        }
                        _ => unreachable!(),
                    }
                }

                println!("{res}");
            }

            _ => unreachable!(),
        }
    }
}
