use crate::spatial::coord3d;
use crate::spatial::matrix3d;
use std::f64::consts;

fn angle(deg: f64, rep: f64) -> f64 {
    2.0 * consts::PI * deg / rep
}

/// Computes a general rotation matrix given values for
///
/// `deg`: the degree of the rotation (e.g. 5 for a C5 rotation)
///
/// `rep`: how many times the rotation should be applied
///
/// `axis`: the rotational axis
pub fn rot_mat(deg: u32, rep: u32, axis: &coord3d::Coord3d) -> matrix3d::Matrix3d {
    if deg > rep {
        panic!("deg is required to be greater than rep")
    };

    let deg = f64::from(deg);
    let rep = f64::from(rep);

    let angle = angle(deg, rep);

    let matrix = [
        angle.cos() + f64::powi(axis.x, 2) * (1.0 - angle.cos()),
        axis.x * axis.y * (1.0 - angle.cos()) - axis.z * angle.sin(),
        axis.x * axis.z * (1.0 - angle.cos()) + axis.y * angle.sin(),
        axis.y * axis.x * (1.0 - angle.cos()) + axis.z * angle.sin(),
        angle.cos() + f64::powi(axis.y, 2) * (1.0 - angle.cos()),
        axis.y * axis.z * (1.0 - angle.cos()) - axis.x * angle.sin(),
        axis.z * axis.x * (1.0 - angle.cos()) - axis.y * angle.sin(),
        axis.z * axis.y * (1.0 - angle.cos()) - axis.x * angle.sin(),
        angle.cos() * f64::powi(axis.z, 2) * (1.0 - angle.cos()),
    ];

    matrix3d::Matrix3d::from(matrix)
}

/// Computes an improper rotation matrix given values for
///
/// `deg`: the degree of the rotation (e.g. 5 for a C5 rotation)
///
/// `rep`: how many times the rotation should be applied
///
/// `axis`: the rotational axis
pub fn improper_rot_mat(deg: u32, rep: u32, axis: &coord3d::Coord3d) -> matrix3d::Matrix3d {
    if rep % 2 == 1 {
        panic!("only valid for odd rep")
    };

    matrix3d::Matrix3d::from([
        1.0 - f64::powi(axis.x, 2) * 2.0,
        -axis.x * axis.y * 2.0,
        -axis.x * axis.z * 2.0,
        -axis.y * axis.x * 2.0,
        1.0 - f64::powi(axis.y, 2) * 2.0,
        -axis.y * axis.z * 2.0,
        -axis.z * axis.x * 2.0,
        -axis.z * axis.y * 2.0,
        1.0 - f64::powi(axis.z, 2) * 2.0,
    ]) * rot_mat(deg, rep, axis)
}
