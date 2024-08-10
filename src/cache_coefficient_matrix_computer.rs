use crate::coefficient_matrix_computer::CoefficientMatrixComputer;
use crate::matrix::Matrix;

pub struct CacheCoefficientMatrixComputer {
    cache: Vec<Vec<u32>>
}

impl CacheCoefficientMatrixComputer {
    pub fn new() -> CacheCoefficientMatrixComputer {
        CacheCoefficientMatrixComputer {
            cache: Vec::default()
        }
    }

    pub fn precompute_cache_for(&mut self, n: usize) {
        if n + 1 <= self.cache.len() {
            return;
        }

        let last_len = self.cache.len();
        self.cache.resize(n + 1, Vec::default());

        for i in last_len..(n + 1) {
            self.cache[i].resize((i + 1) / 2 + 1, 0);
            self.cache[i][0] = 1;
        }

        for i in last_len..(n + 1) {
            for j in 1..((i + 1 + 1) / 2) {
                self.cache[i][j] = self.cache[i - 1][j - 1] + self.cache[i - 1][j];
            }

            if i & 1 == 1 {
                *self.cache[i].last_mut().unwrap() = self.cache[i][i / 2];
            }
        }
    }

    fn get_binomial_coefficient(&self, n: usize, mut k: usize) -> u32 {
        if k > (n + 1) / 2 {
            k = n - k;
        }

        return self.cache[n][k];
    }
}

impl CoefficientMatrixComputer for CacheCoefficientMatrixComputer {
    fn compute_for(&self, n: usize) -> Matrix<f32> {
        let mut result = Matrix::new(n + 1, n + 1);

        for k in 0..=n {
            let factor = self.get_binomial_coefficient(n, k) as f32;

            result.row_at_mut(k).iter_mut().rev().skip(k).enumerate().for_each(|(i, value)| {
                let sign = if i & 1 == 0 { 1 } else { -1 };
                *value = (sign as f32) * factor * (self.get_binomial_coefficient(n - k, i) as f32);
            });
        }

        result
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}
