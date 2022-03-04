pub mod geometry;
mod ops;

#[derive(Debug, Copy, Clone)]
pub struct Coord3d {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Coord3d {
    pub fn array_rep(&self) -> [f64; 3] {
        [self.x, self.y, self.z]
    }
}

use std::fmt;

impl fmt::Display for Coord3d {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "{{{},{},{}}}", self.x, self.y, self.z)
    }
}
