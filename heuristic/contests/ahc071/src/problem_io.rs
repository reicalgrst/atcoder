use proconio::input;

pub struct Input {
    pub W: usize,
    pub H: usize,
    pub c: Vec<isize>,
    pub where_to_fill: Vec<usize>,
}

impl Input {
    pub fn new() -> Self {
        input! {
          W: usize, H: usize, K: usize,
          c: [isize; 5],
          ab: [(usize, usize); K]
        }

        let mut where_to_fill = vec![0; H];
        for (a, b) in ab {
            where_to_fill[b] |= 1_usize << a;
        }

        Self {
            W,
            H,
            c,
            where_to_fill,
        }
    }
}
