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
        n: usize,
        s: Chars,
        t: Chars
    }

    for i in 0..n {
        if s[i] != t[i] && t[i] != '*' {
            println!("No");
            return;
        }
    }

    println!("Yes");
}
