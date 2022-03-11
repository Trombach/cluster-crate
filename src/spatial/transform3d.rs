use crate::spatial::{Coord3d, Matrix3d};
use std::f64::consts;

/// Computes a general rotation matrix given values for
///
/// `DEG`: the degree of the rotation (e.g. 5 for a C5 rotation)
///
/// `REP`: how many times the rotation should be applied
///
/// `axis`: the rotational axis
///
/// # Example
/// ```
/// # use cluster_crate::spatial::Coord3d;
/// # use cluster_crate::spatial::transform3d;
///
/// let rot_axis_normed = Coord3d::from([1.0, 1.0, 1.0]);
/// let rot_mat = transform3d::rot_mat::<5, 1>(&rot_axis_normed);
/// ```
pub fn rot_mat<const DEG: u8, const REP: u8>(axis: &Coord3d) -> Matrix3d {
    assert!(DEG > REP, "deg is required to be greater than rep");

    let angle: f64 = 2.0 * consts::PI * (DEG as f64) / (REP as f64);

    let matrix = [
        angle.cos() + f64::powi(axis[0], 2) * (1.0 - angle.cos()),
        axis[0] * axis[1] * (1.0 - angle.cos()) - axis[2] * angle.sin(),
        axis[0] * axis[2] * (1.0 - angle.cos()) + axis[1] * angle.sin(),
        axis[1] * axis[0] * (1.0 - angle.cos()) + axis[2] * angle.sin(),
        angle.cos() + f64::powi(axis[1], 2) * (1.0 - angle.cos()),
        axis[1] * axis[2] * (1.0 - angle.cos()) - axis[0] * angle.sin(),
        axis[2] * axis[0] * (1.0 - angle.cos()) - axis[1] * angle.sin(),
        axis[2] * axis[1] * (1.0 - angle.cos()) - axis[0] * angle.sin(),
        angle.cos() * f64::powi(axis[2], 2) * (1.0 - angle.cos()),
    ];

    Matrix3d::from(matrix)
}

/// Computes an improper rotation matrix given values for
///
/// `DEG`: the degree of the rotation (e.g. 5 for a C5 rotation)
///
/// `REP`: how many times the rotation should be applied
///
/// `axis`: the rotational axis
///
/// # Example
/// ```
/// # use cluster_crate::spatial;
/// # use cluster_crate::spatial::transform3d;
///
/// let rot_axis_normed = spatial::Coord3d::from([1.0, 1.0, 1.0]);
/// let rot_mat = transform3d::improper_rot_mat::<5, 1>(&rot_axis_normed);
/// ```
pub fn improper_rot_mat<const DEG: u8, const REP: u8>(axis: &Coord3d) -> Matrix3d {
    assert!(REP % 2 == 1, "only valid for odd rep");

    Matrix3d::from([
        1.0 - f64::powi(axis[0], 2) * 2.0,
        -axis[0] * axis[1] * 2.0,
        -axis[0] * axis[2] * 2.0,
        -axis[1] * axis[0] * 2.0,
        1.0 - f64::powi(axis[1], 2) * 2.0,
        -axis[1] * axis[2] * 2.0,
        -axis[2] * axis[0] * 2.0,
        -axis[2] * axis[1] * 2.0,
        1.0 - f64::powi(axis[2], 2) * 2.0,
    ]) * rot_mat::<DEG, REP>(axis)
}

/// Computes and inversion matrix
///
/// # Example
/// ```
/// # use cluster_crate::spatial::transform3d;
///
/// let inversion = transform3d::inversion();
/// ```
pub fn inversion() -> Matrix3d {
    -Matrix3d::new_unit_matrix()
}
