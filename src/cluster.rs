pub mod polyhedron;

use self::polyhedron::Polyhedron;
use crate::spatial::{Coord3d, Coordinates};

#[derive(Debug, PartialEq)]
pub struct Cluster {
    pub coords: Coordinates,
}

impl Cluster {
    pub fn new() -> Self {
        Self {
            coords: Coordinates::new(),
        }
    }

    pub fn size(&self) -> u32 {
        self.coords.size() as u32
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

impl From<Vec<Coord3d>> for Cluster {
    fn from(vec: Vec<Coord3d>) -> Self {
        Self {
            coords: Coordinates::from(vec),
        }
    }
}

impl From<Coordinates> for Cluster {
    fn from(coords: Coordinates) -> Self {
        Self { coords }
    }
}

use std::fmt::Display;

impl Display for Cluster {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}\n{}", self.size(), self.coords)
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        cluster::Cluster,
        spatial::{Coord3d, Coordinates},
    };

    #[test]
    fn create_cluster() {
        let result = Cluster {
            coords: Coordinates::new(),
        };
        assert_eq!(Cluster::new(), result);

        let coords = vec![Coord3d::from([1.0, 2.0, 3.0])];
        let result = Cluster {
            coords: Coordinates::from(vec![Coord3d::from([1.0, 2.0, 3.0])]),
        };
        assert_eq!(Cluster::from(coords), result);
    }
}
