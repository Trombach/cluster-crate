pub mod polyhedron;

pub use self::polyhedron::Polyhedron;
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
            Polyhedron::Tetrahedron(n_layers, scaling) => {
                self::polyhedron::tetrahedron::new(n_layers, scaling)
            }
            Polyhedron::Pyramid(n_layers, scaling) => {
                self::polyhedron::pyramid::new(n_layers, scaling)
            }
            Polyhedron::Icosahedron(n_layers, scaling) => {
                self::polyhedron::icosahedron::new(n_layers, scaling)
            }
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
        cluster::{Cluster, Polyhedron},
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

    #[test]
    fn test_pyr() {
        let result = Cluster {
            coords: Coordinates::from(vec![
                Coord3d::from([0.0, 0.000000, 0.000000]),
                Coord3d::from([2.5, -1.4433756729740645, -3.7788843072031635]),
                Coord3d::from([5.0, -2.886751345948129, -7.557768614406327]),
                Coord3d::from([7.5, -4.3301270189221945, -11.336652921609492]),
                Coord3d::from([0.0, 2.886751345948129, -3.7788843072031635]),
                Coord3d::from([2.5, 1.4433756729740645, -7.557768614406327]),
                Coord3d::from([5.0, 0.0, -11.336652921609492]),
                Coord3d::from([0.0, 5.773502691896258, -7.557768614406327]),
                Coord3d::from([2.5, 4.3301270189221945, -11.336652921609492]),
                Coord3d::from([0.0, 8.660254037844389, -11.336652921609492]),
                Coord3d::from([-2.5, -1.4433756729740645, -3.7788843072031635]),
                Coord3d::from([0.0, -2.886751345948129, -7.557768614406327]),
                Coord3d::from([2.5, -4.3301270189221945, -11.336652921609492]),
                Coord3d::from([-2.5, 1.4433756729740645, -7.557768614406327]),
                Coord3d::from([0.0, 0.0, -11.336652921609492]),
                Coord3d::from([-2.5, 4.3301270189221945, -11.336652921609492]),
                Coord3d::from([-5.0, -2.886751345948129, -7.557768614406327]),
                Coord3d::from([-2.5, -4.3301270189221945, -11.336652921609492]),
                Coord3d::from([-5.0, 0.0, -11.336652921609492]),
                Coord3d::from([-7.5, -4.3301270189221945, -11.336652921609492]),
            ]),
        };

        // The test coordinates require 4 layers and a scaling of 5.0
        assert_eq!(Cluster::from(Polyhedron::Pyramid(4, Some(5.0))), result)
    }

    #[test]
    fn test_tet() {
        let result = Cluster {
            coords: Coordinates::from(vec![
                Coord3d::from([0.0, 0.0, 0.0]),
                Coord3d::from([1.9798989873223327, 1.9798989873223327, 1.9798989873223327]),
                Coord3d::from([3.9597979746446654, 3.9597979746446654, 3.9597979746446654]),
                Coord3d::from([5.939696961966998, 5.939696961966998, 5.939696961966998]),
                Coord3d::from([1.9798989873223327, 0.0, 1.9798989873223327]),
                Coord3d::from([3.9597979746446654, 1.9798989873223327, 3.9597979746446654]),
                Coord3d::from([5.939696961966998, 3.9597979746446654, 5.939696961966998]),
                Coord3d::from([3.9597979746446654, 0.0, 3.9597979746446654]),
                Coord3d::from([5.939696961966998, 1.9798989873223327, 5.939696961966998]),
                Coord3d::from([5.939696961966998, 0.0, 5.939696961966998]),
                Coord3d::from([0.0, 1.9798989873223327, 1.9798989873223327]),
                Coord3d::from([1.9798989873223327, 3.9597979746446654, 3.9597979746446654]),
                Coord3d::from([3.9597979746446654, 5.939696961966998, 5.939696961966998]),
                Coord3d::from([1.9798989873223327, 1.9798989873223327, 3.9597979746446654]),
                Coord3d::from([3.9597979746446654, 3.9597979746446654, 5.939696961966998]),
                Coord3d::from([3.9597979746446654, 1.9798989873223327, 5.939696961966998]),
                Coord3d::from([0.0, 3.9597979746446654, 3.9597979746446654]),
                Coord3d::from([1.9798989873223327, 5.939696961966998, 5.939696961966998]),
                Coord3d::from([1.9798989873223327, 3.9597979746446654, 5.939696961966998]),
                Coord3d::from([0.0, 5.939696961966998, 5.939696961966998]),
            ]),
        };

        // The test coordinates require 4 layers and the default scaling of 2.8
        assert_eq!(Cluster::from(Polyhedron::Tetrahedron(4, None)), result)
    }
}
