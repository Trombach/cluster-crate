use std::ops::{self, Index};

use super::coord3d::Coord3d;

#[derive(Debug, Clone, PartialEq)]
pub struct Coordinates {
    pub values: Vec<Coord3d>,
}

impl Coordinates {
    pub fn add(&mut self, coord: Coordinates) {
        self.values.extend(coord.values);
    }
}

impl ops::Mul<f64> for Coordinates {
    type Output = Coordinates;

    fn mul(self, rhs: f64) -> Self::Output {
        Self::Output {
            values: self.values.into_iter().map(|coord| coord * rhs).collect(),
        }
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

use std::fmt::Display;

impl Display for Coordinates {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for coord in &self.values {
            write!(f, "{}", coord)?;
        }

        Ok(())
    }
}
