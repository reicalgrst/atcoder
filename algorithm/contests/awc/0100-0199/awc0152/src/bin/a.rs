#![allow(non_snake_case)]
use proconio::*;
use superslice::Ext;

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
        _n: usize, m: usize, s: usize, t: usize,
        mut pv: [(usize, usize); m]
    }

    if m == 0 {
        println!("0");
        return;
    }

    let (s, t) = if s > t { (t, s) } else { (s, t) };
    pv.sort_by_key(|(p, _)| *p);

    let l = pv.lower_bound_by_key(&s, |&(p, _)| p);
    let r = pv.upper_bound_by_key(&t, |&(p, _)| p);

    let mut res = 0;
    for i in l..r {
        res += pv[i].1;
    }

    println!("{res}");
}
