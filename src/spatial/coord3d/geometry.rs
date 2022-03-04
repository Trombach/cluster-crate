use super::Coord3d;
use crate::spatial::matrix3d::Matrix3d;

impl Coord3d {
    pub fn dot(&self) -> f64 {
        f64::powi(self.x, 2) + f64::powi(self.y, 2) + f64::powi(self.z, 2)
    }

    pub fn norm(&self) -> f64 {
        self.dot().sqrt()
    }

    pub fn dist(&self, other: &Self) -> f64 {
        (self.clone() - other.clone()).norm()
    }

    // d/dx_i ||x|| = x_i/||x||
    pub fn d_norm(&self) -> Self {
        self.clone() / self.clone().norm()
    }

    pub fn dd_norm(&self) -> Matrix3d {
        let n = 1.0 / self.norm();
        let n3 = f64::powi(n, 3);
        let array_rep = self.array_rep();

        let mut matrix = [0.0; 9];

        for i in 0..3 {
            for j in 0..3 {
                matrix[i * 3 + j] =
                    -array_rep[i] * array_rep[j] * n3 + if i == j { n } else { 0.0 };
            }
        }

        Matrix3d::from(matrix)
    }
}
