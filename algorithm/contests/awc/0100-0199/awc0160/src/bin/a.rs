#![allow(non_snake_case)]
use itertools::Itertools;
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
        n: usize, m: usize,
        s: [[usize; m]; n]
    }

    let mut score = vec![0; n];
    let mut maxi = 0;
    for i in 0..n {
        score[i] = s[i].iter().sorted().skip(1).take(m - 2).sum::<usize>();
        maxi = maxi.max(score[i]);
    }

    println!("{}", score.iter().position(|&v| v == maxi).unwrap() + 1);
}
