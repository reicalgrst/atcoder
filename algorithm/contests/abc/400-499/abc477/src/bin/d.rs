#![allow(non_snake_case)]
use itertools::Itertools;
use proconio::{marker::Usize1, *};
use rustc_hash::FxHashSet;

// https://maguro.dev/blog/debug-macro/
#[allow(unused_macros)]
macro_rules! debug {
    ($($a:expr),* $(,)*) => {
        #[cfg(debug_assertions)]
        eprintln!(concat!($("| ", stringify!($a), " = {:?} "),*, "|"), $(&$a),*);
    };
}

/// クエリ周りのリファクタリング後

enum Query {
    X(usize),
    Color(char),
}

impl Query {
    fn new(q: usize) -> Vec<Query> {
        let mut res = vec![];
        for _ in 0..q {
            input! {
                kind: usize
            }

            if kind == 1 {
                input! {
                    x: Usize1
                }

                res.push(Query::X(x));
            } else {
                input! {
                    c: char
                }

                res.push(Query::Color(c));
            }
        }
        res
    }
}

fn main() {
    input! {
        n: usize, q: usize,
    }

    let ops = Query::new(q);

    let mut covered = vec![false; n];
    for i in 0..q {
        match ops[i] {
            Query::X(x) => {
                covered[x] = !covered[x];
            }
            _ => {}
        }
    }

    let mut set = FxHashSet::default();
    for i in 0..n {
        if !covered[i] {
            set.insert(i);
        }
    }

    let mut res = vec!['a'; n];
    let mut fixed = vec![false; n];
    for i in (0..q).rev() {
        match ops[i] {
            Query::X(x) => {
                if fixed[x] {
                    continue;
                }

                covered[x] = !covered[x];
                if !covered[x] {
                    set.insert(x);
                } else {
                    set.remove(&x);
                }
            }

            Query::Color(c) => {
                for &idx in &set {
                    res[idx] = c;
                    fixed[idx] = true;
                }
                set.clear();
            }
        }
    }

    println!("{}", res.iter().join(""));
}
