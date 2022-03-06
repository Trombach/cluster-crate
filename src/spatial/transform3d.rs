use crate::spatial::coord3d;
use crate::spatial::matrix3d;
use std::f64::consts;

/**
Computes a general rotation matrix given values for

`DEG`: the degree of the rotation (e.g. 5 for a C5 rotation)

`REP`: how many times the rotation should be applied

`axis`: the rotational axis

# Example
```
let rot_mat = rot_mat::<5, 1>(&rot_axis_normed);
```
*/
pub fn rot_mat<const DEG: u8, const REP: u8>(axis: &coord3d::Coord3d) -> matrix3d::Matrix3d {
    assert!(DEG > REP, "deg is required to be greater than rep");

    let angle: f64 = 2.0 * consts::PI * (DEG as f64) / (REP as f64);

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

/**
Computes an improper rotation matrix given values for

`DEG`: the degree of the rotation (e.g. 5 for a C5 rotation)

`REP`: how many times the rotation should be applied

`axis`: the rotational axis

# Example
```
let rot_mat = improper_rot_mat::<5, 1>(&rot_axis_normed);
```
*/
pub fn improper_rot_mat<const DEG: u8, const REP: u8>(
    axis: &coord3d::Coord3d,
) -> matrix3d::Matrix3d {
    assert!(REP % 2 == 1, "only valid for odd rep");

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
    ]) * rot_mat::<DEG, REP>(axis)
}

pub fn inversion() -> matrix3d::Matrix3d {
    -matrix3d::Matrix3d::new_unit_matrix()
}
