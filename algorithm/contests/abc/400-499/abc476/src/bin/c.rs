#![allow(non_snake_case)]
use proconio::*;
use std::{cmp::Reverse, collections::BinaryHeap};

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
        a: [usize; n]
    }

    let mut v = BinaryHeap::<Reverse<usize>>::new();
    for i in 0..3 {
        v.push(Reverse(a[i]));
    }

    println!("{}", v.iter().next().unwrap().0);
    for i in 3..n {
        v.push(Reverse(a[i]));
        v.pop();
        println!("{}", v.iter().next().unwrap().0);
    }
}
