#![allow(non_snake_case)]
use proconio::{marker::Usize1, *};
use std::{
    cmp::{Ord, Ordering, PartialOrd},
    collections::{BinaryHeap, HashMap},
};

// https://maguro.dev/blog/debug-macro/
#[allow(unused_macros)]
macro_rules! debug {
    ($($a:expr),* $(,)*) => {
        #[cfg(debug_assertions)]
        eprintln!(concat!($("| ", stringify!($a), " = {:?} "),*, "|"), $(&$a),*);
    };
}

const INF: usize = usize::MAX;

#[derive(PartialEq, Eq)]
struct Data {
    cost: usize,
    v: usize,
}

impl Ord for Data {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for Data {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn main() {
    input! {
        N: usize, M: usize, S: Usize1,
        uvw: [(Usize1, Usize1, usize); M]
    }

    let mut graph = vec![vec![]; N];
    let mut weight = vec![HashMap::new(); N];

    for (u, v, w) in uvw {
        graph[u].push(v);
        graph[v].push(u);

        weight[u].insert(v, w);
        weight[v].insert(u, w);
    }

    let mut heap = BinaryHeap::<Data>::new();
    heap.push(Data { cost: 0, v: S });
    let mut dist = vec![INF; N];

    while let Some(data) = heap.pop() {
        let (cost, u) = (data.cost, data.v);
        if dist[u] != INF {
            continue;
        }

        dist[u] = cost;
        for &v in &graph[u] {
            let ncost = cost + weight[u].get(&v).unwrap();
            heap.push(Data { cost: ncost, v });
        }
    }

    let res = dist.into_iter().filter(|&u| u != INF).sum::<usize>();
    println!("{res}");
}
