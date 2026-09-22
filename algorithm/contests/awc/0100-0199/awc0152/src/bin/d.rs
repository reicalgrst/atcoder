#![allow(non_snake_case)]
use itertools::Itertools;
use proconio::*;
use std::collections::HashMap;

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
        n: usize, s: u128,
        p: [u128; n]
    }

    if n == 1 {
        if p[0] == s {
            println!("ALMOST");
        } else {
            println!("NO");
        }
        return;
    }

    let a = p.iter().take(n / 2).map(|&u| u).collect_vec();
    let b = p.iter().skip(n / 2).map(|&u| u).collect_vec();

    let amap = count(&a, s);
    let bmap = count(&b, s);

    let mut res = 0;
    for (sum, cnt) in amap {
        if bmap.contains_key(&(s - sum)) {
            res += cnt * bmap[&(s - sum)];
        }
    }

    if res == 0 {
        println!("NO");
    } else if res == 1 {
        println!("ALMOST");
    } else {
        println!("YES");
    }
}

fn count(v: &Vec<u128>, s: u128) -> HashMap<u128, u128> {
    let n = v.len();
    let mut res = HashMap::new();

    for bit in 0..(1 << n) {
        let mut sum = 0;
        for i in 0..n {
            if bit & (1 << i) != 0 {
                sum += v[i];
            }
        }

        if sum <= s {
            *res.entry(sum).or_insert(0) += 1;
        }
    }

    res
}
