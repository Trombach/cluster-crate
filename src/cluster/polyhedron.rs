pub mod icosahedron;
pub mod pyramid;
pub mod tetrahedron;
mod utils;

#[derive(Debug)]
pub enum Polyhedron {
    Tetrahedron(u16, Option<f64>),
    Pyramid(u16, Option<f64>),
    Icosahedron(u16, Option<f64>),
}

use std::fmt::Display;

impl Display for Polyhedron {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            Self::Tetrahedron(i, _) => write!(f, "Tetrahedron, {} layers", i),
            Self::Pyramid(i, _) => write!(f, "Pyramid, {} layers", i),
            Self::Icosahedron(i, _) => write!(f, "Icosahedron, {} layers", i),
        }
    }
}
