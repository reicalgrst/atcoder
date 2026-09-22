#![allow(non_snake_case)]
use itertools::Itertools;
use proconio::*;
use std::cmp::max;

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
        m: usize, d: isize,
        mut lrv: [(isize, isize, isize); m]
    }

    let ls = lrv
        .iter()
        .sorted_by(|&(l1, r1, _), &(l2, r2, _)| l1.cmp(&l2).then(r1.cmp(&r2)))
        .collect_vec();

    let rs = lrv
        .iter()
        .sorted_by(|&(l1, r1, _), &(l2, r2, _)| r2.cmp(&r1).then(l2.cmp(&l1)))
        .collect_vec();

    let mut lcsum = vec![0; m + 1];
    let mut rcsum = vec![0; m + 1];

    for i in 0..m {
        lcsum[i + 1] = lcsum[i] + ls[i].2;
        rcsum[i + 1] = rcsum[i] + rs[i].2;
    }

    let mut res = 0;

    // 区間の左端と右端をチェックポイントにして全探索するとよい.
    // 左 → 右
    for i in 0..m {
        let l = ls[i].0;

        let mut sum = 0;
        for j in i..m {
            let r = ls[j].1;
            if r - l + l.abs() <= d {
                res = max(res, lcsum[j + 1] - lcsum[i] - sum);
            } else {
                sum += ls[j].2;
            }
        }
    }

    // 右 → 左
    for i in 0..m {
        let r = rs[i].1;

        let mut sum = 0;
        for j in i..m {
            let l = rs[j].0;
            if r - l + r.abs() <= d {
                res = max(res, rcsum[j + 1] - rcsum[i] - sum);
            } else {
                sum += rs[j].2;
            }
        }
    }

    println!("{res}");
}
