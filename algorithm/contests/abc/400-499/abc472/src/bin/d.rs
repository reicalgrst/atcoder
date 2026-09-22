#![allow(non_snake_case)]
use itertools::iproduct;
use proconio::{marker::Chars, *};

// https://maguro.dev/blog/debug-macro/
macro_rules! debug {
    ($($a:expr),* $(,)*) => {
        #[cfg(debug_assertions)]
        eprintln!(concat!($("| ", stringify!($a), "={:?} "),*, "|"), $(&$a),*);
    };
}

fn main() {
    input! {
        h: usize, w: usize, k: usize,
        s: [Chars; h]
    }

    let mut safe_r = vec![];
    let mut safe_c = vec![];

    // 安全な行
    for y in 0..h {
        let mut ok = true;
        for x in 0..w {
            if s[y][x] == '#' {
                ok = false;
            }
        }

        if ok {
            safe_c.push(y);
        }
    }

    // 安全な列
    for x in 0..w {
        let mut ok = true;
        for y in 0..h {
            if s[y][x] == '#' {
                ok = false;
            }
        }

        if ok {
            safe_r.push(x);
        }
    }

    let mut visited = vec![vec![false; w]; h];
    let mut queue = std::collections::VecDeque::new();

    for (x, y) in iproduct!(safe_r, safe_c) {
        queue.push_back((x, y, 0));
        visited[y][x] = true;
    }

    let dx = vec![1, 0, -1, 0];
    let dy = vec![0, 1, 0, -1];

    debug!(queue);

    while let Some((x, y, cnt)) = queue.pop_front() {
        assert!(s[y][x] != '#');

        for i in 0..4 {
            let nx = x as isize + dx[i];
            let ny = y as isize + dy[i];

            if nx < 0 || w as isize <= nx || ny < 0 || h as isize <= ny {
                continue;
            }

            let (nx, ny) = (nx as usize, ny as usize);
            if cnt + 1 <= k && !visited[ny][nx] && s[ny][nx] != '#' {
                queue.push_back((nx, ny, cnt + 1));
                visited[ny][nx] = true;
            }
        }
    }

    let res = visited.iter().flatten().filter(|&&u| u).count();
    println!("{res}");
}
