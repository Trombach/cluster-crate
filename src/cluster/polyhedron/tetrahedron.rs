use super::{utils::non_cart_coord, Polyhedron};
use crate::{
    cluster::Cluster,
    spatial::{coordinates::Coordinates, matrix3d::Matrix3d},
};

pub fn new(n_layers: &u16) -> Cluster {
    Cluster {
        coords: generate_coords(&n_layers),
        kind: Polyhedron::Tetrahedron,
    }
}

fn generate_coords(n_layers: &u16) -> Coordinates {
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
    let non_cart_coord = non_cart_coord(n_layers);

    trans_matrix * non_cart_coord
}
