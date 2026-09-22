#![allow(non_snake_case)]
use proconio::*;

fn main() {
    input! {
        n: usize,
        l: [isize; n]
    }

    let mut res = isize::MAX;
    let mut sum = 0;
    let all = l.iter().sum::<isize>();

    for i in 0..n {
        sum += l[i];
        let rem = all - sum;
        res = std::cmp::min(res, (sum - rem).abs());
    }

    println!("{res}");
}
