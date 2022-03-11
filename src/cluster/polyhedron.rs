pub mod icosahedron;
pub mod pyramid;
pub mod tetrahedron;
mod utils;

#[derive(Debug)]
pub enum Polyhedron {
    Tetrahedron(u16),
    Pyramid(u16),
    Icosahedron(u16),
}

use std::fmt::Display;

impl Display for Polyhedron {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            Self::Tetrahedron(i) => write!(f, "Tetrahedron, {} layers", i),
            Self::Pyramid(i) => write!(f, "Pyramid, {} layers", i),
            Self::Icosahedron(i) => write!(f, "Icosahedron, {} layers", i),
        }
    }
}
