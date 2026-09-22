/// dp で行ごとの最適解が求まるらしいので試してみる
/// 何もないところにも置けるように変更
use crate::problem_io::*;
use ahc_library::random::Random;
use bitset_fixed::BitSet;
use itertools::iproduct;

const WIDTHS: [usize; 5] = [1, 3, 5, 7, 9];

#[derive(Clone)]
pub struct State {
    pub bricks: Vec<Vec<(usize, usize, usize)>>,
    pub masks: Vec<usize>,
    pub score: isize,
}

impl State {
    fn row_opt(&mut self, input: &Input, Y: usize) {
        let mut dp = vec![isize::MAX; input.W + 1];
        let mut row_opt = vec![BitSet::new(5 * input.W + 1); input.W + 1];
        dp[0] = 0;

        for x in 0..input.W {
            if self.masks[Y] & (1 << x) == 0 {
                if dp[x] < dp[x + 1] {
                    dp[x + 1] = dp[x];
                    row_opt[x + 1] = row_opt[x].clone();
                }
            }

            for idx in 0..WIDTHS.len() {
                let width = WIDTHS[idx];
                if input.W < x + width {
                    continue;
                }

                let width_mask = ((1_usize << width) - 1) << x;
                if self.masks[Y] & width_mask == 0 {
                    continue;
                }

                let nx = x + width;
                if dp[x] + input.c[idx] < dp[nx] {
                    dp[nx] = dp[x] + input.c[idx];
                    row_opt[nx] = row_opt[x].clone();
                    row_opt[nx].set(input.W * idx + x, true);
                }
            }
        }

        for (x, idx) in iproduct!(0..input.W, 0..WIDTHS.len()) {
            let bit = input.W * idx + x;
            if row_opt[input.W][bit] {
                if 0 < Y {
                    self.masks[Y - 1] |= 1 << (x + idx);
                }
                self.score += input.c[idx];
                self.bricks[Y].push((x, Y, WIDTHS[idx]));
            }
        }
    }

    /// ランダムなレンガの長さを ± 2 にしてみる
    fn row_width_random(&mut self, input: &Input, Y: usize, rng: &mut Random) -> Result<(), ()> {
        for &(_, _, w) in &self.bricks[Y] {
            self.score -= input.c[w / 2];
        }

        let mut legal_change = vec![];
        for (idx, &(x, _, old_width)) in self.bricks[Y].iter().enumerate() {
            for &width in &WIDTHS {
                if width.abs_diff(old_width) != 2 {
                    continue;
                }

                if x + width <= input.W {
                    legal_change.push((idx, width));
                }
            }
        }

        let len = legal_change.len();
        if len == 0 {
            return Err(());
        }

        let change_idx = rng.usize(0, len);
        let (bricks_idx, new_width) = legal_change[change_idx];
        let (start_x, _, _) = self.bricks[Y][bricks_idx];
        self.bricks[Y].truncate(bricks_idx);

        let mut cost = 0;
        for j in 0..bricks_idx {
            cost += input.c[self.bricks[Y][j].2 / 2];
        }

        self.bricks[Y].push((start_x, Y, new_width));
        cost += input.c[new_width / 2];
        let nx = start_x + new_width;

        let mut dp = vec![isize::MAX; input.W + 1];
        let mut row_opt = vec![BitSet::new(5 * input.W + 1); input.W + 1];
        dp[nx] = cost;

        for &(x, _, w) in &self.bricks[Y] {
            let bit = input.W * (w / 2) + x;
            row_opt[nx].set(bit, true);
        }

        for x in nx..input.W {
            if self.masks[Y] & (1 << x) == 0 {
                if dp[x] < dp[x + 1] {
                    dp[x + 1] = dp[x];
                    row_opt[x + 1] = row_opt[x].clone();
                }
            }

            for idx in 0..WIDTHS.len() {
                let width = WIDTHS[idx];

                if input.W < x + width {
                    continue;
                }

                let width_mask = ((1_usize << width) - 1) << x;
                if self.masks[Y] & width_mask == 0 {
                    continue;
                }

                let nx = x + width;

                if dp[x] + input.c[idx] < dp[nx] {
                    dp[nx] = dp[x] + input.c[idx];
                    row_opt[nx] = row_opt[x].clone();
                    row_opt[nx].set(input.W * idx + x, true);
                }
            }
        }

        self.bricks[Y].clear();
        for (x, idx) in iproduct!(0..input.W, 0..WIDTHS.len()) {
            let bit = input.W * idx + x;
            if row_opt[input.W][bit] {
                if 0 < Y {
                    self.masks[Y - 1] |= 1 << (x + idx);
                }
                self.score += input.c[idx];
                self.bricks[Y].push((x, Y, WIDTHS[idx]));
            }
        }

        Ok(())
    }

    /// ランダムなレンガを左右に動かす
    fn row_shift_random(&mut self, input: &Input, Y: usize, rng: &mut Random) -> Result<(), ()> {
        let n = self.bricks[Y].len();
        if n == 0 {
            return Err(());
        }

        let i = rng.usize(0, n);
        let (old_x, _, w) = self.bricks[Y][i];
        let possible_left = if i == 0 {
            0
        } else {
            let (px, _, pw) = self.bricks[Y][i - 1];
            px + pw
        };

        let possible_right = if i + 1 == n {
            input.W - w
        } else {
            let (nx, _, _) = self.bricks[Y][i + 1];
            if nx < w {
                return Err(());
            }
            nx - w
        };

        let mut mask_without = 0;
        for (j, &(x, _, w)) in self.bricks[Y].iter().enumerate() {
            if j == i {
                continue;
            }
            mask_without |= ((1_usize << w) - 1) << x;
        }

        if possible_left > possible_right {
            return Err(());
        }

        let mut legal_change = vec![];
        for new_x in possible_left..=possible_right {
            if new_x == old_x {
                continue;
            }

            let new_mask = ((1_usize << self.bricks[Y][i].2) - 1) << new_x;
            if self.masks[Y] & !(new_mask | mask_without) == 0 {
                legal_change.push(new_x);
            }
        }

        if legal_change.len() == 0 {
            return Err(());
        }

        let new_x = legal_change[rng.usize(0, legal_change.len())];
        self.bricks[Y][i].0 = new_x;

        Ok(())
    }

    /// Y 未満を貪欲で作る
    fn greedy_below(&mut self, input: &Input, Y: usize) {
        for y in (0..Y).rev() {
            self.row_opt(input, y);
        }
    }

    pub fn output(&self) {
        println!("{}", self.bricks.iter().map(|v| v.len()).sum::<usize>());
        for &(x, y, w) in self.bricks.iter().flatten() {
            println!("{x} {y} {w}");
        }
    }
}

pub fn greedy(input: &Input) -> State {
    let mut initial_state = State {
        bricks: vec![vec![]; input.H],
        masks: input.where_to_fill.clone(),
        score: 0,
    };

    initial_state.greedy_below(&input, input.H);
    initial_state
}

/// 幅をずらす
pub fn width_randomizer(
    input: &Input,
    state: &State,
    rng: &mut Random,
    Y: usize,
) -> Result<State, ()> {
    let mut res = state.clone();
    // Y 未満を初期化する
    for y in 0..Y {
        for &(_, _, w) in &res.bricks[y] {
            res.score -= input.c[w / 2];
        }
        res.bricks[y].clear();
        res.masks[y] = input.where_to_fill[y];
    }

    if let Ok(_) = res.row_width_random(input, Y, rng) {
        res.greedy_below(input, Y);
        Ok(res)
    } else {
        Err(())
    }
}

/// 置く場所をずらす
pub fn shift_randomizer(
    input: &Input,
    state: &State,
    rng: &mut Random,
    Y: usize,
) -> Result<State, ()> {
    // 意味がない
    if Y == 0 {
        return Err(());
    }

    let mut res = state.clone();
    for y in 0..Y {
        for &(_, _, w) in &res.bricks[y] {
            res.score -= input.c[w / 2];
        }
        res.bricks[y].clear();
        res.masks[y] = input.where_to_fill[y];
    }

    if let Ok(()) = res.row_shift_random(input, Y, rng) {
        for &(x, _, w) in &res.bricks[Y] {
            res.masks[Y - 1] |= 1_usize << (x + w / 2);
        }

        res.greedy_below(input, Y);
        Ok(res)
    } else {
        Err(())
    }
}
