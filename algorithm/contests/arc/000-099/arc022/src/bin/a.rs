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

    let (mut is_i, mut is_c, mut is_t) = (false, false, false);
    for c in s {
        if c.to_ascii_uppercase() == 'I' {
            is_i = true;
        }

        if is_i && c.to_ascii_uppercase() == 'C' {
            is_c = true;
        }

        if is_c && c.to_ascii_uppercase() == 'T' {
            is_t = true;
        }
    }

    let res = if is_t { "YES" } else { "NO" };
    println!("{res}");
}
