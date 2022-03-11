use crate::cluster::Cluster;
use crate::spatial::{Coordinates, Matrix3d};
use std::f64::consts::PI;

pub fn new(n_layers: u16) -> Cluster {
    Cluster::from(generate_coords(&n_layers))
}

fn generate_coords(n_layers: &u16) -> Coordinates {
    trans_matrix() * super::utils::non_cart_coord(n_layers) * 5.0
}

fn trans_matrix() -> Matrix3d {
    Matrix3d::new(
        -0.5,
        0.0,
        0.5,
        -1.0 / (2.0 * 3_f64.sqrt()),
        1.0 / 3_f64.sqrt(),
        -1.0 / (2.0 * 3_f64.sqrt()),
        -(2.0 * PI / 11.0).sqrt(),
        -(2.0 * PI / 11.0).sqrt(),
        -(2.0 * PI / 11.0).sqrt(),
    )
}
