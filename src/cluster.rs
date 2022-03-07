pub mod polyhedron;

use self::polyhedron::Polyhedron;
use crate::spatial::coordinates::Coordinates;

#[derive(Debug)]
pub struct Cluster {
    pub coords: Coordinates,
    pub kind: Polyhedron,
}

impl Cluster {
    pub fn new(polyhedron: Polyhedron, n_layers: u16) -> Self {
        match polyhedron {
            Polyhedron::Tetrahedron => self::polyhedron::tetrahedron::new(n_layers),
            Polyhedron::Icosahedron => self::polyhedron::icosahedron::new(n_layers),
        }
    }

    pub fn size(&self) -> u32 {
        self.coords.values[..].len() as u32
    }
}

use std::fmt::Display;

impl Display for Cluster {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}\n{}\n{}", self.size(), self.kind, self.coords)
    }
}
