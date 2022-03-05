use crate::spatial::coord3d::Coord3d;
use crate::spatial::coordinates::Coordinates;
use crate::spatial::matrix3d::Matrix3d;

pub struct Cluster {
    pub coords: Coordinates,
    pub kind: Polyhedron,
}

pub enum Polyhedron {
    Tetrahedron,
    Icosahedron,
}

impl Cluster {
    pub fn new(polyhedron: Polyhedron, n_layers: &u16) -> Cluster {
        match polyhedron {
            Polyhedron::Tetrahedron => Self::new_tetrahedron(n_layers),
            Polyhedron::Icosahedron => Self::new_icosahedron(n_layers),
        }
    }

    fn new_tetrahedron(n_layers: &u16) -> Cluster {
        Cluster {
            coords: Self::generate_tetrahedron_coords(&n_layers),
            kind: Polyhedron::Tetrahedron,
        }
    }

    fn generate_tetrahedron_coords(n_layers: &u16) -> Coordinates {
        let trans_matrix = Matrix3d::new(
            0.0,
            1.0 / 2_f64.sqrt(),
            1.0 / 2_f64.sqrt(),
            1.0 / 2_f64.sqrt(),
            0.0,
            1.0 / 2_f64.sqrt(),
            1.0 / 2_f64.sqrt(),
            1.0 / 2_f64.sqrt(),
            0.0,
        );
        let non_cart_coord = Self::non_cart_coord(n_layers);

        trans_matrix * non_cart_coord
    }

    fn new_icosahedron(n_layers: &u16) -> Cluster {
        todo!()
    }

    fn non_cart_coord(n_layers: &u16) -> Coordinates {
        let mut vector = Vec::<Coord3d>::new();

        for k in 0..*n_layers {
            let l = 0;
            while k + l < *n_layers {
                let m = 0;
                while k + l + m < *n_layers {
                    vector.push(Coord3d {
                        x: f64::from(k),
                        y: f64::from(l),
                        z: f64::from(m),
                    })
                }
            }
        }

        Coordinates::from(vector)
    }
}
