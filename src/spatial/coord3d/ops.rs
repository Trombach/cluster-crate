use super::Coord3d;
use std::ops::{self, Index, IndexMut};

impl ops::AddAssign<Coord3d> for Coord3d {
    fn add_assign(&mut self, rhs: Self) {
        self[0] += rhs[0];
        self[1] += rhs[1];
        self[2] += rhs[2];
    }
}

impl ops::Add<Coord3d> for Coord3d {
    type Output = Self;

    fn add(mut self, rhs: Self) -> Self {
        self += rhs;
        self
    }
}

impl ops::SubAssign<Coord3d> for Coord3d {
    fn sub_assign(&mut self, rhs: Self) {
        self[0] -= rhs[0];
        self[1] -= rhs[1];
        self[2] -= rhs[2];
    }
}

impl ops::Sub<Coord3d> for Coord3d {
    type Output = Self;

    fn sub(mut self, rhs: Self) -> Self {
        self -= rhs;
        self
    }
}

impl ops::MulAssign<f64> for Coord3d {
    fn mul_assign(&mut self, rhs: f64) {
        self[0] *= rhs;
        self[1] *= rhs;
        self[2] *= rhs;
    }
}

impl ops::Mul<f64> for Coord3d {
    type Output = Self;

    fn mul(mut self, rhs: f64) -> Self {
        self *= rhs;
        self
    }
}

impl ops::DivAssign<f64> for Coord3d {
    fn div_assign(&mut self, rhs: f64) {
        self[0] /= rhs;
        self[1] /= rhs;
        self[2] /= rhs;
    }
}

impl ops::Div<f64> for Coord3d {
    type Output = Self;

    fn div(mut self, rhs: f64) -> Self {
        self /= rhs;
        self
    }
}

impl ops::Neg for Coord3d {
    type Output = Self;

    fn neg(self) -> Self {
        Self([-self[0], -self[1], -self[2]])
    }
}

impl Index<usize> for Coord3d {
    type Output = f64;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl IndexMut<usize> for Coord3d {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}

#[cfg(test)]
mod tests {
    use crate::spatial::coord3d::Coord3d;

    #[test]
    fn add_coord3d() {
        let result = Coord3d::from([1.0, 2.0, 3.0]);
        let to_add = Coord3d::from([1.0, 3.0, 2.0]);

        let mut coord = Coord3d::from([0.0, -1.0, 1.0]);

        assert_eq!(coord + to_add, result);

        coord += to_add;

        assert_eq!(result, coord);
    }

    #[test]
    fn sub_coord3d() {
        let result = Coord3d::from([-1.0, -4.0, -1.0]);
        let to_sub = Coord3d::from([1.0, 3.0, 2.0]);

        let mut coord = Coord3d::from([0.0, -1.0, 1.0]);

        assert_eq!(coord - to_sub, result);

        coord -= to_sub;

        assert_eq!(result, coord);
    }

    #[test]
    fn mul_f64_coord3d() {
        let result = Coord3d::from([0.0, -2.0, 2.0]);
        let to_mul = 2.0;

        let mut coord = Coord3d::from([0.0, -1.0, 1.0]);

        assert_eq!(coord * to_mul, result);

        coord *= to_mul;

        assert_eq!(result, coord);
    }

    #[test]
    fn mdiv_f64_coord3d() {
        let result = Coord3d::from([0.0, -0.5, 0.5]);
        let to_div = 2.0;

        let mut coord = Coord3d::from([0.0, -1.0, 1.0]);

        assert_eq!(coord / to_div, result);

        coord /= to_div;

        assert_eq!(result, coord);
    }

    #[test]
    fn neg_coord3d() {
        let result = Coord3d::from([1.0, 2.0, 3.0]);
        let coord = Coord3d::from([-1.0, -2.0, -3.0]);

        assert_eq!(-coord, result);
    }
}
