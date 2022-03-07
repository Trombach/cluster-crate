pub mod geometry;
mod ops;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Coord3d {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Coord3d {
    pub fn as_array(&self) -> [f64; 3] {
        [self.x, self.y, self.z]
    }
}

impl From<[f64; 3]> for Coord3d {
    fn from(array: [f64; 3]) -> Self {
        Coord3d {
            x: array[0],
            y: array[1],
            z: array[2],
        }
    }
}

use std::fmt;

impl fmt::Display for Coord3d {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "{{{:.8},{:.8},{:.8}}}", self.x, self.y, self.z)
    }
}

#[cfg(test)]
mod tests {
    use crate::spatial::coord3d::Coord3d;

    #[test]
    fn as_array() {
        let result = [2.0, 3.0, 1.0];

        assert_eq!(
            Coord3d {
                x: 2.0,
                y: 3.0,
                z: 1.0
            }
            .as_array(),
            result
        );
    }

    #[test]
    fn from_array() {
        let result = Coord3d {
            x: 2.0,
            y: 3.0,
            z: 1.0,
        };

        assert_eq!(Coord3d::from([2.0, 3.0, 1.0]), result);
    }
}
