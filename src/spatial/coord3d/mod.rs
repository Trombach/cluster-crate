pub mod geometry;
mod ops;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Coord3d([f64; 3]);

impl Coord3d {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { 0: [x, y, z] }
    }
}

impl From<[f64; 3]> for Coord3d {
    fn from(array: [f64; 3]) -> Self {
        Coord3d(array)
    }
}

impl Into<[f64; 3]> for Coord3d {
    fn into(self) -> [f64; 3] {
        self.0
    }
}

impl Into<[f64; 3]> for &Coord3d {
    fn into(self) -> [f64; 3] {
        self.0
    }
}

use std::fmt;

impl fmt::Display for Coord3d {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "{:.8}  {:.8}  {:.8}", self[0], self[1], self[2])
    }
}

#[cfg(test)]
mod tests {
    use crate::spatial::Coord3d;

    #[test]
    fn as_array() {
        let result = [2.0, 3.0, 1.0];
        let coord: [f64; 3] = Coord3d::new(2.0, 3.0, 1.0).into();

        assert_eq!(coord, result);
    }

    #[test]
    fn from_array() {
        let result = Coord3d { 0: [2.0, 3.0, 1.0] };

        assert_eq!(Coord3d::from([2.0, 3.0, 1.0]), result);
    }
}
