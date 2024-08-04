use std::ops::{AddAssign, Mul};

#[derive(Debug, Clone, Default)]
pub struct Matrix<T: Clone> {
    pub data: Vec<T>,
    pub rows: usize,
    pub columns: usize
}

impl<T> Matrix<T> where T: Default + Clone {
    pub fn new(rows: usize, columns: usize) -> Matrix<T> {
        Matrix {
            data: vec![T::default(); rows * columns],
            rows,
            columns
        }
    }

    pub fn from_table(table: &[&[T]]) -> Matrix<T> {
        Matrix {
            data: table.iter().flat_map(|row| row.iter()).map(|v| v.clone()).collect(),
            rows: table.len(),
            columns: table[0].len()
        }
    }

    pub fn row_at(&self, i: usize) -> &[T] {
        &self.data[(i * self.columns)..((i + 1) * self.columns)]
    }

    pub fn row_at_mut(&mut self, i: usize) -> &mut [T] {
        &mut self.data[(i * self.columns)..((i + 1) * self.columns)]
    }
}

impl<T> Matrix<T> where T: Default + Copy + Mul<Output = T> + AddAssign<T> {
    pub fn multiply(&self, other: &Matrix<T>) -> Option<Matrix<T>> {
        if self.columns != other.rows {
            return None;
        }

        return Some(self.multiply_unchecked(other));
    }

    pub fn multiply_unchecked(&self, other: &Matrix<T>) -> Matrix<T> {
        let mut result = Matrix::new(self.rows, other.columns);

        for i in 0..result.rows {
            for j in 0..result.columns {
                for k in 0..self.columns {
                    result.data[i * result.columns + j] += self.data[i * self.columns + k] * other.data[k * other.columns + j];
                }
            }
        }

        result
    }
}
