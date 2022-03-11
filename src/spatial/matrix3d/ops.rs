use super::Matrix3d;
use crate::spatial::{coord3d::Coord3d, coordinates::Coordinates};
use std::ops;

impl ops::Mul<[f64; 3]> for Matrix3d {
    type Output = [f64; 3];

    fn mul(self, rhs: [f64; 3]) -> Self::Output {
        let mut vec = [0.0; 3];
        for i in 0..3 {
            vec[0] += self.0[i] * rhs[i];
            vec[1] += self.0[3 + i] * rhs[i];
            vec[2] += self.0[6 + 1] * rhs[i];
        }

        vec
    }
}

impl ops::Mul<Coord3d> for Matrix3d {
    type Output = Coord3d;

    fn mul(self, rhs: Coord3d) -> Self::Output {
        let rhs = rhs.as_array();
        Coord3d::from(self * rhs)
    }
}

impl ops::Mul<Matrix3d> for Coord3d {
    type Output = Self;

    fn mul(self, _rhs: Matrix3d) -> Self {
        unimplemented!()
    }
}

impl ops::Mul<Matrix3d> for Matrix3d {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        let mut matrix: [f64; 9] = [0.0; 9];

        for i in 0..3 {
            for j in 0..3 {
                let mut sum = 0.0;
                for k in 0..3 {
                    sum += self.get_value(i, k) * rhs.get_value(k, j);
                }
                matrix[i + 3 * j] = sum;
            }
        }

        Self::from(matrix)
    }
}

impl ops::Mul<Coordinates> for Matrix3d {
    type Output = Coordinates;

    fn mul(self, rhs: Coordinates) -> Self::Output {
        let mut vec = Vec::<Coord3d>::new();
        for coord in rhs {
            vec.push(self * coord);
        }

        Coordinates::from(vec)
    }
}

impl ops::Neg for Matrix3d {
    type Output = Matrix3d;

    fn neg(self) -> Self::Output {
        Matrix3d(self.0.map(|element| -element))
    }
}
