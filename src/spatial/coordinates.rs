mod ops;

#[derive(Debug, Clone, PartialEq)]
pub struct Coordinates(Vec<Coord3d>);

impl Coordinates {
    pub fn new(coord: Vec<Coord3d>) -> Self {
        Self(coord)
    }

    pub fn add(&mut self, coord: Coordinates) {
        self.0.extend(coord.0);
    }

    pub fn size(&self) -> usize {
        self.0.len()
    }
}

impl From<Vec<Coord3d>> for Coordinates {
    fn from(vec: Vec<Coord3d>) -> Coordinates {
        Coordinates { 0: vec }
    }
}

impl IntoIterator for Coordinates {
    type Item = Coord3d;

    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

use std::fmt::Display;

use super::coord3d::Coord3d;

impl Display for Coordinates {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for coord in &self.0 {
            write!(f, "{}", coord)?;
        }

        Ok(())
    }
}
