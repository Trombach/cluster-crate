use crate::spatial::{Coord3d, Coordinates};
use std::ops::{self, Index, IndexMut};

impl ops::Mul<f64> for Coordinates {
    type Output = Coordinates;

    fn mul(self, rhs: f64) -> Self::Output {
        Self::Output {
            0: self.0.into_iter().map(|coord| coord * rhs).collect(),
        }
    }
}

impl Index<usize> for Coordinates {
    type Output = Coord3d;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl IndexMut<usize> for Coordinates {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}
