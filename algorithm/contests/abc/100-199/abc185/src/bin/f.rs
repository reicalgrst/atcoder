#![allow(non_snake_case)]
use ac_library::segtree::{Monoid, Segtree};
use proconio::*;

struct S;
impl Monoid for S {
    type S = usize;

    fn identity() -> Self::S {
        0
    }

    fn binary_operation(a: &Self::S, b: &Self::S) -> Self::S {
        a ^ b
    }
}

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
        n: usize, q: usize,
        a: [usize; n]
    }

    let mut segtree = Segtree::<S>::from_iter(a);

    for _ in 0..q {
        input! {
            t: usize, x: usize, y: usize
        }

        match t {
            1 => {
                let v = segtree.get(x - 1);
                segtree.set(x - 1, v ^ y);
            }
            2 => {
                let res = segtree.prod(x - 1..y);
                println!("{res}");
            }
            _ => unreachable!(),
        }
    }
}
