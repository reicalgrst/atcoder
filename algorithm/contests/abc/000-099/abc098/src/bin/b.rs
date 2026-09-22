#![allow(non_snake_case)]
use proconio::{marker::Chars, *};
use rustc_hash::FxHashSet;

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
        n: usize, s: Chars
    }

    let mut res = 0;
    for i in 0..n {
        let pre = FxHashSet::from_iter(s.iter().take(i));
        let suf = FxHashSet::from_iter(s.iter().skip(i));

        let mut cnt = 0;
        for &&c in &pre {
            if suf.contains(&c) {
                cnt += 1;
            }
        }

        res = res.max(cnt);
    }

    println!("{res}");
}
