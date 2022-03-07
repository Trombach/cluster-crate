pub mod icosahedron;
pub mod tetrahedron;
mod utils;

#[derive(Debug)]
pub enum Polyhedron {
    Tetrahedron,
    Icosahedron,
}

use std::fmt::Display;

impl Display for Polyhedron {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            Self::Tetrahedron => write!(f, "Tetrahedron"),
            Self::Icosahedron => write!(f, "Icosahedron"),
        }
    }
}
