pub mod icosahedron;
pub mod pyramid;
pub mod tetrahedron;
mod utils;

#[derive(Debug)]
pub enum Polyhedron {
    Tetrahedron,
    Pyramid,
    Icosahedron,
}

use std::fmt::Display;

impl Display for Polyhedron {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            Self::Tetrahedron => write!(f, "Tetrahedron"),
            Self::Pyramid => write!(f, "Pyramid"),
            Self::Icosahedron => write!(f, "Icosahedron"),
        }
    }
}
