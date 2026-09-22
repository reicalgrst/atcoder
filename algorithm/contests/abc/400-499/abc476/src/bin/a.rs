#![allow(non_snake_case)]
use itertools::Itertools;
use proconio::{marker::Chars, *};

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
        t: Chars
    }

    if *t.last().unwrap() == 'e' {
        println!("{}r", t.iter().join(""));
    } else {
        println!("{}er", t.iter().join(""));
    }
}
