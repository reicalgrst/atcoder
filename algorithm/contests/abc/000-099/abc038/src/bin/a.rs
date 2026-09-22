#![allow(non_snake_case)]
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
        s: Chars
    }

    let res = if *s.last().unwrap() == 'T' {
        "YES"
    } else {
        "NO"
    };

    println!("{res}");
}
