pub mod polyhedron;

use self::polyhedron::Polyhedron;
use crate::spatial::coordinates::Coordinates;

#[derive(Debug)]
pub struct Cluster(Coordinates);

impl Cluster {
    pub fn new(coords: Coordinates) -> Self {
        Self(coords)
    }

    pub fn size(&self) -> u32 {
        self.0.size() as u32
    }
}

impl From<Polyhedron> for Cluster {
    fn from(polyhedron: Polyhedron) -> Self {
        match polyhedron {
            Polyhedron::Tetrahedron(i) => self::polyhedron::tetrahedron::new(i),
            Polyhedron::Pyramid(i) => self::polyhedron::pyramid::new(i),
            Polyhedron::Icosahedron(i) => self::polyhedron::icosahedron::new(i),
        }
    }
}

use std::fmt::Display;

impl Display for Cluster {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}\n{}", self.size(), self.0)
    }
}
