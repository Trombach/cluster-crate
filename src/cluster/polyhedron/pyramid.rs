use super::utils;
use crate::cluster::{Cluster, Coordinates};
use crate::spatial::Matrix3d;
use std::f64::consts::PI;

pub fn new(n_layers: u16, scaling: Option<f64>) -> Cluster {
    Cluster::from(
        trans_matrix()
            * utils::non_cart_coord(n_layers)
                .into_iter()
                .map(|c| c * scaling.unwrap_or(2.8))
                .collect::<Coordinates>(),
    )
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
