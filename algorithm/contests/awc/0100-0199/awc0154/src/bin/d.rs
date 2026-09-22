#![allow(non_snake_case)]
use itertools::{Itertools, iproduct};
use proconio::{marker::Usize1, *};
use std::cmp::min;

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
        n: usize, m: usize,
        uvw: [(Usize1, Usize1, usize); m]
    }

    let mut graph = vec![vec![]; n];
    let mut dist = vec![vec![INF; n]; n];

    for i in 0..m {
        let (u, v, w) = uvw[i];
        graph[u].push(v);
        dist[u][v] = w;
    }

    for (k, i, j) in iproduct!(0..n, 0..n, 0..n) {
        if dist[i][k] == INF || dist[k][j] == INF {
            continue;
        }
        dist[i][j] = min(dist[i][j], dist[i][k] + dist[k][j]);
    }

    let mut cnt = vec![0; n];
    for (s, g) in iproduct!(0..n, 0..n).filter(|(u, v)| u != v) {
        if dist[s][g] == INF {
            continue;
        }

        for v in (0..n).filter(|&u| u != s && u != g) {
            if dist[s][v] == INF || dist[v][g] == INF {
                continue;
            }

            if dist[s][v] + dist[v][g] == dist[s][g] {
                cnt[v] += 1;
            }
        }
    }

    println!("{}", cnt.iter().join("\n"));
}
