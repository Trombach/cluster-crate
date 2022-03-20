use super::utils;
use crate::{cluster::Cluster, spatial::matrix3d::Matrix3d};

pub fn new(n_layers: u16, scaling: Option<f64>) -> Cluster {
    Cluster::from(trans_matrix() * utils::non_cart_coord(n_layers) * scaling.unwrap_or(2.8))
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
