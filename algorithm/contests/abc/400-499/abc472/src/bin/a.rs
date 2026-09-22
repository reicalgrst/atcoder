#![allow(non_snake_case)]
use proconio::{marker::Chars, *};

fn main() {
    input! {
        mut s: Chars
    }

    let res = s
        .iter()
        .map(|&c| if c != 'A' { '.' } else { c })
        .collect::<String>();
    println!("{res}");
}
