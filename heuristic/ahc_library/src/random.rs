use rand::{Rng, SeedableRng};
use rand_pcg::Pcg64Mcg;

pub struct Random(Pcg64Mcg);

impl Random {
    pub fn new(seed: u64) -> Self {
        Self(Pcg64Mcg::seed_from_u64(seed))
    }

    /// [l, r)
    pub fn usize(&mut self, l: usize, r: usize) -> usize {
        self.0.random_range(l..r)
    }

    /// [0, 1)
    pub fn f64(&mut self) -> f64 {
        self.0.random()
    }

    /// True or False
    pub fn bool(&mut self) -> bool {
        self.0.random()
    }
}
