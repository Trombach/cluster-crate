use crate::spatial::coord3d;

pub struct Matrix3d {
    values: [f64; 9],
}

impl Matrix3d {
    pub fn from(values: [f64; 9]) -> Matrix3d {
        Self { values }
    }

    pub fn new(
        x0: f64,
        x1: f64,
        x2: f64,
        x3: f64,
        x4: f64,
        x5: f64,
        x6: f64,
        x7: f64,
        x8: f64,
    ) -> Self {
        Self {
            values: [x0, x1, x2, x3, x4, x5, x6, x7, x8],
        }
    }

    pub fn new_zero_matrix() -> Matrix3d {
        Self { values: [0.0; 9] }
    }

    pub fn new_unit_matrix() -> Matrix3d {
        Self::new(1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0)
    }

    pub fn get_value(&self, i: usize, j: usize) -> f64 {
        self.values[i + 3 * j]
    }

    pub fn set_value(&mut self, i: usize, j: usize, value: f64) {
        self.values[i + 3 * j] = value;
    }
}

use std::ops;

impl ops::Mul<coord3d::Coord3d> for Matrix3d {
    type Output = Self;

    fn mul(self, other: coord3d::Coord3d) -> Self {
        unimplemented!()
    }
}

impl ops::Mul<Matrix3d> for coord3d::Coord3d {
    type Output = Self;

    fn mul(self, other: Matrix3d) -> Self {
        unimplemented!()
    }
}

impl ops::Mul<Self> for Matrix3d {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        let mut matrix: [f64; 9] = [0.0; 9];

        for i in 0..3 {
            for j in 0..3 {
                let mut sum = 0.0;
                for k in 0..3 {
                    sum += self.get_value(i, k) * other.get_value(k, j);
                }
                matrix[i + 3 * j] = sum;
            }
        }

        Self::from(matrix)
    }
}
