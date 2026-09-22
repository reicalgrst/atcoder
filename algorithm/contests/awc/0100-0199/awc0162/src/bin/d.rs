#![allow(non_snake_case)]
use itertools::iproduct;
use proconio::{marker::*, *};
use rustc_hash::FxHashSet;
use std::{cmp::Reverse, collections::BinaryHeap};

// https://maguro.dev/blog/debug-macro/
#[allow(unused_macros)]
macro_rules! debug {
    ($($a:expr),* $(,)*) => {
        #[cfg(debug_assertions)]
        eprintln!(concat!($("| ", stringify!($a), " = {:?} "),*, "|"), $(&$a),*);
    };
}

const INF: usize = usize::MAX;

fn main() {
    input! {
        h: usize, w: usize,
        sy: Usize1, sx: Usize1,
        g: [Chars; h],
        n: usize,
        s: Chars
    }

    let set = FxHashSet::from_iter(g.iter().flatten().clone());

    for i in 0..n {
        if !set.contains(&&s[i]) {
            println!("-1");
            return;
        }
    }

    let dx = vec![0, 1, 0, !0];
    let dy = vec![1, 0, !0, 0];

    // dp[i][j] := (i, j) にいるときの最小コスト
    let mut dp = vec![vec![INF; w]; h];
    dp[sy][sx] = 0;

    for i in 0..n {
        let mut heapq = BinaryHeap::new();
        let mut dist = vec![vec![INF; w]; h];

        for (x, y) in iproduct!(0..w, 0..h) {
            if dp[y][x] < INF {
                heapq.push(Reverse((dp[y][x], x, y)));
                dist[y][x] = dp[y][x];
            }
        }

        while let Some(Reverse((cost, x, y))) = heapq.pop() {
            if dist[y][x] != cost {
                continue;
            }

            for i in 0..4 {
                let (nx, ny) = (x.wrapping_add(dx[i]), y.wrapping_add(dy[i]));

                if w <= nx || h <= ny {
                    continue;
                }

                let ncost = cost + 1;
                if ncost < dist[ny][nx] {
                    dist[ny][nx] = ncost;
                    heapq.push(Reverse((ncost, nx, ny)));
                }
            }
        }

        for (x, y) in iproduct!(0..w, 0..h) {
            if g[y][x] == s[i] {
                dp[y][x] = dist[y][x];
            } else {
                dp[y][x] = INF;
            }
        }
    }

    let res = dp.into_iter().flatten().min().unwrap() + n;
    println!("{res}");
}
