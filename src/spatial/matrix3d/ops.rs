use super::Matrix3d;
use crate::spatial::{coord3d::Coord3d, coordinates::Coordinates};
use std::ops;

impl ops::Mul<Coord3d> for Matrix3d {
    type Output = Coord3d;

    fn mul(self, rhs: Coord3d) -> Coord3d {
        let rhs = rhs.as_array();
        let mut coord3d = Coord3d::from([0.0; 3]);
        for i in 0..3 {
            coord3d += Coord3d::from([
                self.values[i] * rhs[i],
                self.values[3 + i] * rhs[i],
                self.values[6 + i] * rhs[i],
            ]);
        }

        coord3d
    }
}

impl ops::Mul<Matrix3d> for Coord3d {
    type Output = Self;

    fn mul(self, rhs: Matrix3d) -> Self {
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
        Matrix3d {
            values: self.values.map(|element| -element),
        }
    }
}
