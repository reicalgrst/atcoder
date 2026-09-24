#![allow(non_snake_case)]
use ac_library::segtree::{Max, Segtree};
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
        n: usize, t: usize,
        s: [usize; n]
    }

    let mut res = 0;
    let st = Segtree::<Max<usize>>::from_iter(s.clone());

    for i in 1..n {
        let maxi = st.prod(0..i);
        if t <= maxi && s[i] < maxi {
            res += 1;
        }
    }

    println!("{res}");
}
