#![allow(non_snake_case)]
use itertools::Itertools;
use proconio::*;
use std::collections::HashMap;
use superslice::Ext;

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
        n: usize, k: usize, x: isize,
        mut a: [isize; n]
    }

    let b = a.split_off(n / 2);
    let mut a_map = vec![HashMap::new(); a.len() + 1];
    let mut b_map = vec![HashMap::new(); b.len() + 1];

    dfs(0, 0, 0, &mut a_map, &a);
    dfs(0, 0, 0, &mut b_map, &b);

    let a_map = a_map
        .into_iter()
        .map(|map| map.into_iter().sorted().collect_vec())
        .collect_vec();

    let b_map = b_map
        .into_iter()
        .map(|map| map.into_iter().sorted().collect_vec())
        .collect_vec();

    let mut b_csum = vec![vec![]; b_map.len()];
    for i in 0..b_map.len() {
        let v = &b_map[i];
        b_csum[i] = vec![0; v.len() + 1];
        for j in 0..v.len() {
            b_csum[i][j + 1] = b_csum[i][j] + v[j].1;
        }
    }

    let mut res = 0;
    for cnt in k.saturating_sub(b.len())..=k.min(a.len()) {
        let rem = k - cnt;
        for &(sum, v) in &a_map[cnt] {
            let idx = b_map[rem].lower_bound_by(|&(bsum, _)| bsum.cmp(&(x - sum)));

            res += (b_csum[rem].last().unwrap() - b_csum[rem][idx]) * v;
        }
    }

    println!("{res}");
}

fn dfs(depth: usize, cnt: usize, sum: isize, map: &mut Vec<HashMap<isize, usize>>, v: &Vec<isize>) {
    if depth == v.len() {
        *map[cnt].entry(sum).or_insert(0) += 1;
        return;
    }

    dfs(depth + 1, cnt + 1, sum + v[depth], map, v);
    dfs(depth + 1, cnt, sum, map, v);
}
