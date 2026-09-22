#![allow(non_snake_case)]
use ac_library::SccGraph;
use proconio::{marker::Usize1, *};
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
        n: usize, m: usize,
        w: [isize; n],
        t: [isize; n],
        uv: [(Usize1, Usize1); m]
    }

    let mut need = vec![0; n];
    for i in 0..n {
        need[i] = max(0, t[i] - w[i]);
    }

    let mut graph = SccGraph::new(n);
    for (u, v) in uv {
        graph.add_edge(u, v);
    }

    let scc = graph.scc();
    let k = scc.len();

    // 不可能かの判定
    for c in 0..k {
        if scc[c].len() == 1 {
            for &v in &scc[c] {
                if need[v] != 0 {
                    println!("-1");
                    return;
                }
            }
        }
    }

    let mut res = 0;
    for c in 0..k {
        let mut cnt = 0;
        for &v in &scc[c] {
            cnt = max(cnt, need[v]);
        }
        res += cnt;
    }
    println!("{res}");
}
