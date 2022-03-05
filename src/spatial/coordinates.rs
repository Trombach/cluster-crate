use super::coord3d::Coord3d;

pub struct Coordinates {
    values: Vec<Coord3d>,
}

impl From<Vec<Coord3d>> for Coordinates {
    fn from(vec: Vec<Coord3d>) -> Coordinates {
        Coordinates { values: vec }
    }
}

// impl Iterator for Coordinates {
//     type Item = Coord3d;

//     fn next(&mut self) -> Option<Self::Item> {
//         self.values.next()
//     }
// }

impl IntoIterator for Coordinates {
    type Item = Coord3d;

    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.values.into_iter()
    }
}
