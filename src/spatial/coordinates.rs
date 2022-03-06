use std::ops::Index;

use super::coord3d::Coord3d;

#[derive(Debug, Clone)]
pub struct Coordinates {
    pub values: Vec<Coord3d>,
}

impl Coordinates {
    pub fn add(&mut self, mut coord: Coordinates) {
        self.values.extend(coord.values);
    }
}

impl From<Vec<Coord3d>> for Coordinates {
    fn from(vec: Vec<Coord3d>) -> Coordinates {
        Coordinates { values: vec }
    }
}

impl Index<usize> for Coordinates {
    type Output = Coord3d;

    fn index(&self, index: usize) -> &Self::Output {
        &self.values[index]
    }
}

impl IntoIterator for Coordinates {
    type Item = Coord3d;

    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.values.into_iter()
    }
}
