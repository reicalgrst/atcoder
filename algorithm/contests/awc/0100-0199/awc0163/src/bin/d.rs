#![allow(non_snake_case)]
use ac_library::ModInt998244353 as mint;
use itertools::iproduct;
use proconio::{marker::Usize1, *};

// https://maguro.dev/blog/debug-macro/
#[allow(unused_macros)]
macro_rules! debug {
    ($($a:expr),* $(,)*) => {
        #[cfg(debug_assertions)]
        eprintln!(concat!($("| ", stringify!($a), " = {:?} "),*, "|"), $(&$a),*);
    };
}

const OFFSET: isize = 60;

fn main() {
    input! {
        n: usize, p: usize, q: usize, m: usize,
        mut lr: [(usize, usize); n],
        xab: [(Usize1, usize, usize); m]
    }

    for i in 0..m {
        let (x, a, b) = xab[i];
        lr[x] = (a, b);

        let mut dp = vec![vec![vec![mint::new(0); 121]; q + 1]; p + 1];
        dp[0][0][OFFSET as usize] += 1;

        for store in 0..n {
            let (l, r) = lr[store];
            let mut ndp = vec![vec![vec![mint::new(0); 121]; q + 1]; p + 1];

            for (i, j, cost) in iproduct!(0..=p, 0..=q, -60..=60) {
                let base = dp[i][j][(cost + OFFSET) as usize];
                ndp[i][j][(cost + OFFSET) as usize] += base;

                for add in l..=r {
                    if i < p {
                        let ncost = cost + OFFSET + add as isize;
                        if ncost < 0 || 120 < ncost {
                            continue;
                        }
                        ndp[i + 1][j][ncost as usize] += base;
                    }

                    if j < q {
                        let ncost = cost + OFFSET - add as isize;
                        if ncost < 0 || 120 < ncost {
                            continue;
                        }
                        ndp[i][j + 1][ncost as usize] += base;
                    }
                }
            }
            dp = ndp;
        }

        println!("{}", dp[p][q][OFFSET as usize]);
    }
}
