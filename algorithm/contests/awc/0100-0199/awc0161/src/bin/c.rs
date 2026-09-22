#![allow(non_snake_case)]
use proconio::*;
use rustc_hash::FxHashMap;

// https://maguro.dev/blog/debug-macro/
#[allow(unused_macros)]
macro_rules! debug {
    ($($a:expr),* $(,)*) => {
        #[cfg(debug_assertions)]
        eprintln!(concat!($("| ", stringify!($a), " = {:?} "),*, "|"), $(&$a),*);
    };
}

fn chmax(res: &mut usize, other: usize) {
    if *res < other {
        *res = other;
    }
}

fn main() {
    input! {
        n: usize, k: usize,
        t: [usize; n]
    }

    let mut map = FxHashMap::default();
    let mut cnt = 0;
    let mut res = 0;
    let mut r = 0;

    for l in 0..n {
        while r < n && cnt + map.get(&t[r]).is_some() as usize <= k {
            if map.get(&t[r]).is_some() {
                cnt += 1;
            }

            *map.entry(t[r]).or_insert(0) += 1;
            r += 1;
        }

        chmax(&mut res, map.len());

        if l == r {
            r += 1;
        } else {
            let x = map.get_mut(&t[l]).unwrap();
            if *x == 1 {
                map.remove(&t[l]);
            } else {
                *x -= 1;
                cnt -= 1;
            }
        }
    }

    println!("{res}");
}
