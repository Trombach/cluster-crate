mod ops;

#[derive(Debug, Clone, PartialEq)]
pub struct Coordinates(Vec<Coord3d>);

impl Coordinates {
    pub fn new() -> Self {
        Self(Vec::<Coord3d>::new())
    }

    pub fn add(&mut self, coord: Coordinates) {
        self.0.extend(coord.0);
    }

    pub fn add_coord3d(&mut self, coord3d: Coord3d) {
        self.0.push(coord3d);
    }

    pub fn size(&self) -> usize {
        self.0.len()
    }

    pub fn norm(&self) -> Self {
        Self {
            0: self.0.iter().map(|coord| coord.d_norm()).collect(),
        }
    }

    pub fn iter_floats(&self) -> impl Iterator<Item = f64> + '_ {
        self.0.iter().flat_map(|c3| c3.as_array())
    }
}

impl From<Coord3d> for Coordinates {
    fn from(coord: Coord3d) -> Self {
        Coordinates { 0: vec![coord] }
    }
}

impl From<Vec<Coord3d>> for Coordinates {
    fn from(vec: Vec<Coord3d>) -> Self {
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

#[cfg(test)]
mod tests {
    use super::Coordinates;
    use crate::spatial::Coord3d;

    #[test]
    fn create_coordinates() {
        let result = Coordinates {
            0: Vec::<Coord3d>::new(),
        };
        assert_eq!(Coordinates::new(), result);

        let result = Coordinates {
            0: vec![Coord3d::from([1.0, 2.0, 3.0])],
        };
        assert_eq!(
            Coordinates::from(vec![Coord3d::from([1.0, 2.0, 3.0])]),
            result
        )
    }
}
