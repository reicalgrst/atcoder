#![allow(non_snake_case)]
use num::integer::gcd;
use proconio::*;

// https://maguro.dev/blog/debug-macro/
#[allow(unused_macros)]
macro_rules! debug {
    ($($a:expr),* $(,)*) => {
        #[cfg(debug_assertions)]
        eprintln!(concat!($("| ", stringify!($a), " = {:?} "),*, "|"), $(&$a),*);
    };
}

struct RollingHash {
    mod_: u128,
    pow: Vec<u128>,
    hash: Vec<u128>,
}

impl RollingHash {
    fn new(s: &String, base: u128, mod_: u128) -> Self {
        let n = s.len();
        let mut pow = vec![1; n + 1];
        let mut hash = vec![0; n + 1];

        for (i, c) in s.chars().enumerate() {
            pow[i + 1] = pow[i] * base % mod_;
            hash[i + 1] = (hash[i] * base + c as u128) % mod_;
        }

        Self { mod_, pow, hash }
    }

    fn get(&self, l: usize, r: usize) -> u128 {
        (self.hash[r] + self.mod_ - self.hash[l] * self.pow[r - l] % self.mod_) % self.mod_
    }
}

fn solve() {
    input! {
        s: String
    }

    let n = s.len();
    let rh = RollingHash::new(&s, 131, 1_000_000_007);
    let mut res = vec![];
    for len in 1..n {
        if rh.get(0, len) == rh.get(n - len, n) {
            res.push(len);
        }
    }

    if res.len() == 0 {
        println!("0")
    } else if res.len() == 1 {
        println!("{}", res[0]);
    } else {
        let res = res
            .windows(2)
            .fold(res[1] - res[0], |prev, v| gcd(prev, v[1] - v[0]));
        println!("{res}");
    }
}

fn main() {
    input! {
        q: usize
    }

    for _ in 0..q {
        solve();
    }
}
