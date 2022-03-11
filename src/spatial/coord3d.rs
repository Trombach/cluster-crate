pub mod geometry;
mod ops;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Coord3d([f64; 3]);

impl Coord3d {
    pub fn as_array(&self) -> [f64; 3] {
        self.0
    }
}

impl From<[f64; 3]> for Coord3d {
    fn from(array: [f64; 3]) -> Self {
        Coord3d(array)
    }
}

use std::fmt;

impl fmt::Display for Coord3d {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "{{{:.8},{:.8},{:.8}}}", self[0], self[0], self[0])
    }
}

#[cfg(test)]
mod tests {
    use crate::spatial::Coord3d;

    #[test]
    fn as_array() {
        let result = [2.0, 3.0, 1.0];

        assert_eq!(Coord3d { 0: [2.0, 3.0, 1.0] }.as_array(), result);
    }

    #[test]
    fn from_array() {
        let result = Coord3d { 0: [2.0, 3.0, 1.0] };

        assert_eq!(Coord3d::from([2.0, 3.0, 1.0]), result);
    }
}
