#![allow(non_snake_case)]
use im_rc::HashMap;
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
        n: usize, k: isize,
        a: [isize; n]
    }

    let mut map = HashMap::new();
    for i in 0..n {
        *map.entry(a[i]).or_insert(0) += 1;
    }

    let mut res = 0_usize;
    for (&x, &c1) in &map {
        if let Some(c2) = map.get(&(x + k)) {
            res += c1 * c2;
        }

        if let Some(c2) = map.get(&(x - k)) {
            res += c1 * c2;
        }
    }

    res /= 2;
    println!("{res}");
}
