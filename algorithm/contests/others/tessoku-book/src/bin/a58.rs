#![allow(non_snake_case)]
use ac_library::segtree::{Max, Segtree};
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
        n: usize, q: usize
    }

    let mut segtree = Segtree::<Max<isize>>::from_iter(std::iter::repeat(0).take(n));

    for _ in 0..q {
        input! {
            kind: usize
        }

        match kind {
            1 => {
                input! {
                    pos: Usize1, x: isize
                }
                segtree.set(pos, x);
            }
            2 => {
                input! {
                    l: Usize1, r: Usize1
                }

                println!("{}", segtree.prod(l..r));
            }
            _ => unreachable!(),
        }
    }
}
