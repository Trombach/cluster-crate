pub mod polyhedron;

use crate::spatial::coordinates::Coordinates;

use self::polyhedron::Polyhedron;

pub struct Cluster {
    pub coords: Coordinates,
    pub kind: Polyhedron,
}

impl Cluster {
    pub fn new(polyhedron: Polyhedron, n_layers: &u16) -> Self {
        match polyhedron {
            Polyhedron::Tetrahedron => self::polyhedron::tetrahedron::new(n_layers),
            Polyhedron::Icosahedron => self::polyhedron::icosahedron::new(n_layers),
        }
    }
}
