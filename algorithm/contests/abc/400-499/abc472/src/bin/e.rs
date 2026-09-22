#![allow(non_snake_case)]
use itertools::Itertools;
use proconio::{marker::Usize1, *};
use recur_fn::{RecurFn, recur_fn};
use std::cell::RefCell;

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
        t: usize,
    }

    for _ in 0..t {
        solve();
    }
}

fn solve() {
    input! {
        n: usize, m: usize,
        ab: [(Usize1, Usize1); m]
    }

    let mut graph = vec![vec![]; n];
    for (a, b) in ab {
        graph[a].push(b);
        graph[b].push(a);
    }

    let seen = RefCell::new(vec![None; n]);
    seen.borrow_mut()[0] = Some(0);

    let res = RefCell::new(vec![]);
    let vx = RefCell::new(None);
    let is_end = RefCell::new(false);

    let dfs = recur_fn(|dfs, u: usize| -> Result<(), ()> {
        let ucol = seen.borrow()[u].unwrap();
        for &v in &graph[u] {
            if *is_end.borrow() {
                return Ok(());
            }

            if let Some(vcol) = seen.borrow()[v] {
                if vcol == ucol {
                    *vx.borrow_mut() = Some(v);
                    res.borrow_mut().push(u + 1);
                    return Err(());
                }
            } else {
                seen.borrow_mut()[v] = Some(1 - ucol);
                if dfs(v).is_err() {
                    res.borrow_mut().push(u + 1);

                    if Some(u) == *vx.borrow() {
                        *is_end.borrow_mut() = true;
                        return Ok(());
                    }

                    return Err(());
                }
            }
        }

        return Ok(());
    });

    let _ = dfs.call(0);

    if res.borrow().len() == 0 {
        println!("-1");
    } else {
        println!("{}", res.borrow().len());
        println!("{}", res.borrow().iter().join(" "));
    }
}
