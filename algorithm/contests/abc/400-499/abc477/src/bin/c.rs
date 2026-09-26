#![allow(non_snake_case)]
use proconio::{
    marker::{Chars, Usize1},
    *,
};
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
        q: usize, s: Chars, t: Chars
    }

    if s.len() < t.len() {
        for _ in 0..q {
            println!("No");
        }
        return;
    }

    let mut start = vec![];
    for i in 0..s.len() - t.len() + 1 {
        if s[i..i + t.len()] == t {
            start.push(i);
        }
    }

    debug!(start);
    for _ in 0..q {
        input! {
            l: Usize1, r: usize,
        }

        let idx = start.lower_bound(&l);
        if idx < start.len() && start[idx] + t.len() <= r {
            println!("Yes");
        } else {
            println!("No");
        }
    }
}
