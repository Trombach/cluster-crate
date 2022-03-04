use super::Coord3d;
use std::ops;

impl ops::AddAssign<Coord3d> for Coord3d {
    fn add_assign(&mut self, other: Self) {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
    }
}

impl ops::Add<Coord3d> for Coord3d {
    type Output = Self;

    fn add(mut self, other: Self) -> Self {
        self += other;
        self
    }
}

impl ops::SubAssign<Coord3d> for Coord3d {
    fn sub_assign(&mut self, other: Self) {
        self.x -= other.x;
        self.y -= other.y;
        self.z -= other.z;
    }
}

impl ops::Sub<Coord3d> for Coord3d {
    type Output = Self;

    fn sub(mut self, other: Self) -> Self {
        self -= other;
        self
    }
}

impl ops::MulAssign<f64> for Coord3d {
    fn mul_assign(&mut self, other: f64) {
        self.x *= other;
        self.y *= other;
        self.z *= other;
    }
}

impl ops::Mul<f64> for Coord3d {
    type Output = Self;

    fn mul(mut self, other: f64) -> Self {
        self *= other;
        self
    }
}

impl ops::DivAssign<f64> for Coord3d {
    fn div_assign(&mut self, other: f64) {
        self.x /= other;
        self.y /= other;
        self.z /= other;
    }
}

impl ops::Div<f64> for Coord3d {
    type Output = Self;

    fn div(mut self, other: f64) -> Self {
        self /= other;
        self
    }
}

impl ops::Neg for Coord3d {
    type Output = Self;

    fn neg(self) -> Self {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}
