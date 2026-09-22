#![allow(non_snake_case)]
use ac_library::segtree::{Monoid, Segtree};
use proconio::{marker::Usize1, *};

struct S;
impl Monoid for S {
    type S = usize;

    fn identity() -> Self::S {
        0
    }

    fn binary_operation(a: &Self::S, b: &Self::S) -> Self::S {
        a + b
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
        n: usize, q: usize
    }

    let mut segtree = Segtree::<S>::from_iter(std::iter::repeat(0).take(n));

    for _ in 0..q {
        input! {
            kind: usize
        }

        match kind {
            1 => {
                input! {
                    pos: Usize1, x: usize
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
