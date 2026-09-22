#![allow(non_snake_case)]
use proconio::{marker::Chars, *};

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
        n: usize,
        s: [Chars; n]
    }

    let mut max_cnt = 0;
    let mut res = 1;

    for i in 0..n {
        let t = &s[i];

        let mut cnt = 0;
        let m = t.len();
        for j in 0..m {
            if t[j] != 't' {
                continue;
            }

            if m <= j + 7 {
                break;
            }

            let st = t.iter().skip(j).take(8).collect::<String>();
            if st.as_str() == "tanabata" {
                cnt += 1;
            }
        }

        if max_cnt < cnt {
            max_cnt = cnt;
            res = i + 1;
        }
    }

    println!("{res}");
}
