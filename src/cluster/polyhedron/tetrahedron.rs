use super::utils::non_cart_coord;
use crate::{
    cluster::Cluster,
    spatial::{coordinates::Coordinates, matrix3d::Matrix3d},
};

pub fn new(n_layers: u16) -> Cluster {
    Cluster::from(generate_coords(&n_layers))
}

fn generate_coords(n_layers: &u16) -> Coordinates {
    trans_matrix() * non_cart_coord(n_layers) * 2.8
}

fn trans_matrix() -> Matrix3d {
    Matrix3d::new(
        0.0,
        1.0 / 2_f64.sqrt(),
        1.0 / 2_f64.sqrt(),
        1.0 / 2_f64.sqrt(),
        0.0,
        1.0 / 2_f64.sqrt(),
        1.0 / 2_f64.sqrt(),
        1.0 / 2_f64.sqrt(),
        0.0,
    )
}
