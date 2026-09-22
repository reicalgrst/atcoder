#![allow(non_snake_case)]
use ordered_float::OrderedFloat;
use proconio::*;
use std::f64::consts::PI;

// https://maguro.dev/blog/debug-macro/
#[allow(unused_macros)]
macro_rules! debug {
    ($($a:expr),* $(,)*) => {
        #[cfg(debug_assertions)]
        eprintln!(concat!($("| ", stringify!($a), " = {:?} "),*, "|"), $(&$a),*);
    };
}

fn arg(p1: (isize, isize), p2: (isize, isize)) -> f64 {
    let dx = p2.0 - p1.0;
    let dy = p2.1 - p1.1;

    let theta = (dy as f64).atan2(dx as f64);
    if 0.0 <= theta {
        theta
    } else {
        theta + 2.0 * PI
    }
}

fn diff(v: OrderedFloat<f64>) -> OrderedFloat<f64> {
    OrderedFloat((OrderedFloat(PI) - v).abs())
}

fn main() {
    input! {
        n: usize,
        xy: [(isize, isize); n]
    }

    let mut res = OrderedFloat(0.0);

    for i in 0..n {
        let p1 = xy[i];
        let mut v = vec![];

        for j in (0..n).filter(|&u| u != i) {
            v.push(OrderedFloat(arg(p1, xy[j])));
        }

        v.sort();

        let m = n - 1;
        let mut r = 0;

        for l in 0..m {
            while r + 1 < m && diff(v[r] - v[l]) > diff(v[r + 1] - v[l]) {
                r += 1;
            }

            if diff(v[r] - v[l]) < diff(res) {
                res = v[r] - v[l];
            }

            if l == r {
                r += 1;
            }
        }
    }

    res *= (180.0) / PI;
    if OrderedFloat(180.0) < res {
        res = OrderedFloat(360.0) - res;
    }
    println!("{res}");
}
