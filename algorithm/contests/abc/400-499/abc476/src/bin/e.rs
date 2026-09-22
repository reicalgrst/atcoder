#![allow(non_snake_case)]
use ac_library::segtree::{Monoid, Segtree};
use itertools::Itertools;
use proconio::{marker::Usize1, *};

// https://maguro.dev/blog/debug-macro/
#[allow(unused_macros)]
macro_rules! debug {
    ($($a:expr),* $(,)*) => {
        #[cfg(debug_assertions)]
        eprintln!(concat!($("| ", stringify!($a), " = {:?} "),*, "|"), $(&$a),*);
    };
}

// セグメント木にインデックスを持たせてスワップする
// https://yuiga.dev/blog/posts/abc217_e_segment/

const BIG: i128 = 10_i128.pow(9);

struct Min;
impl Monoid for Min {
    type S = i128;

    fn identity() -> Self::S {
        i128::MAX
    }

    fn binary_operation(a: &Self::S, b: &Self::S) -> Self::S {
        if (*a % BIG) <= (*b % BIG) { *a } else { *b }
    }
}

struct Max;
impl Monoid for Max {
    type S = i128;

    fn identity() -> Self::S {
        i128::MIN
    }

    fn binary_operation(a: &Self::S, b: &Self::S) -> Self::S {
        if (*a % BIG) <= (*b % BIG) { *b } else { *a }
    }
}

fn cnv(idx: usize, v: i128) -> i128 {
    v + (idx as i128) * BIG
}

fn main() {
    input! {
        n: usize, m: usize,
        p: [i128; n],
        lr: [(Usize1, Usize1); m]
    }

    let mut min_segtree = Segtree::<Min>::new(n);
    let mut max_segtree = Segtree::<Max>::new(n);

    for i in 0..n {
        min_segtree.set(i, cnv(i, p[i]));
        max_segtree.set(i, cnv(i, p[i]));
    }

    for i in 0..m {
        let (l, r) = lr[i];

        let mini = min_segtree.prod(l..=r);
        let maxi = max_segtree.prod(l..=r);

        let mini_idx = (mini / BIG) as usize;
        let maxi_idx = (maxi / BIG) as usize;

        min_segtree.set(mini_idx, cnv(mini_idx, maxi % BIG));
        min_segtree.set(maxi_idx, cnv(maxi_idx, mini % BIG));
        max_segtree.set(mini_idx, cnv(mini_idx, maxi % BIG));
        max_segtree.set(maxi_idx, cnv(maxi_idx, mini % BIG));
    }

    let mut res = vec![0; n];
    for i in 0..n {
        res[i] = min_segtree.get(i) % BIG;
    }

    println!("{}", res.iter().join(" "));
}
