#![allow(non_snake_case)]
use proconio::*;

fn main() {
    input! {
        n: usize, m: usize, k: usize,
        a: [usize; n]
    }

    let mut sum = 0;
    let mut eat = vec![false; n];

    for i in 0..n {
        if sum + a[i] <= k {
            println!("Yes");
            eat[i] = true;
            sum += a[i];
        } else {
            println!("No");
        }

        if m <= i + 1 && eat[i + 1 - m] {
            sum -= a[i + 1 - m];
        }
    }
}
