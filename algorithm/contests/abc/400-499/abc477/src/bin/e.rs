#![allow(non_snake_case)]
use ac_library::segtree::{Min, Segtree};
use proconio::{marker::Usize1, *};
use std::{cmp::Reverse, collections::BinaryHeap};

// https://maguro.dev/blog/debug-macro/
#[allow(unused_macros)]
macro_rules! debug {
    ($($a:expr),* $(,)*) => {
        #[cfg(debug_assertions)]
        eprintln!(concat!($("| ", stringify!($a), " = {:?} "),*, "|"), $(&$a),*);
    };
}

// リファクタリング後

fn dijkstra(v: usize, a: &Vec<usize>, b: &Vec<usize>, dist: &mut Vec<usize>) {
    let mut pq = BinaryHeap::new();
    pq.push((Reverse(0), v));

    let n = a.len();
    while let Some((Reverse(cst), u)) = pq.pop() {
        if dist[u] < cst {
            continue;
        }

        if u != n {
            let idx1 = (u + 1) % n;
            let ncst1 = a[u] + cst;
            if ncst1 < dist[idx1] {
                dist[idx1] = ncst1;
                pq.push((Reverse(ncst1), idx1));
            }

            let idx2 = (n + u - 1) % n;
            let ncst2 = a[idx2] + cst;
            if ncst2 < dist[idx2] {
                dist[idx2] = ncst2;
                pq.push((Reverse(ncst2), idx2));
            }
        } else {
            for j in 0..n {
                let ncst = cst + b[j];
                if ncst < dist[j] {
                    dist[j] = ncst;
                    pq.push((Reverse(ncst), j));
                }
            }
        }
    }
}

fn main() {
    input! {
        n: usize, q: usize,
        a: [usize; n],
        b: [usize; n]
    }

    let mut dist = vec![usize::MAX; n + 1];
    dist[n] = 0;
    dijkstra(n, &a, &b, &mut dist);

    let mut csum = vec![0; 2 * n + 1];
    for i in 0..2 * n {
        csum[i + 1] = csum[i] + a[i % n];
    }

    let mut ccsum = vec![0; 2 * n];
    for i in 0..2 * n {
        ccsum[i] = csum[i] + dist[i % n];
    }

    let ccsum = Segtree::<Min<usize>>::from(ccsum);

    for _ in 0..q {
        input! {
            s: Usize1, t: Usize1
        }

        if t == n {
            println!("{}", dist[s]);
            continue;
        }

        debug!(s, t);
        let mini1 = ccsum.prod(s..t) - csum[s];
        let d1 = mini1 + dist[t];
        debug!(d1, mini1);

        let mini2 = ccsum.prod(t..(s + n)) - csum[t];
        let d2 = mini2 + dist[s];
        debug!(d2, mini2);

        let d3 = csum[t] - csum[s];
        debug!(d3, csum[t], csum[s]);
        let d4 = csum[s + n] - csum[t];
        debug!(d4, csum[s + n], csum[t]);

        let res = vec![d1, d2, d3, d4].into_iter().min().unwrap();
        println!("{res}");
    }
}
