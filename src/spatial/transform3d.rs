use crate::spatial::coord3d;
use crate::spatial::matrix3d;
use std::f64::consts;

const fn angle(deg: f64, rep: f64) -> f64 {
    2.0 * consts::PI * deg / rep
}

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
    if DEG > REP {
        panic!("deg is required to be greater than rep")
    };

    const DEG: f64 = f64::from(DEG);
    const REP: f64 = f64::from(REP);

    const ANGLE: f64 = angle(DEG, REP);

    let matrix = [
        ANGLE.cos() + f64::powi(axis.x, 2) * (1.0 - ANGLE.cos()),
        axis.x * axis.y * (1.0 - ANGLE.cos()) - axis.z * ANGLE.sin(),
        axis.x * axis.z * (1.0 - ANGLE.cos()) + axis.y * ANGLE.sin(),
        axis.y * axis.x * (1.0 - ANGLE.cos()) + axis.z * ANGLE.sin(),
        ANGLE.cos() + f64::powi(axis.y, 2) * (1.0 - ANGLE.cos()),
        axis.y * axis.z * (1.0 - ANGLE.cos()) - axis.x * ANGLE.sin(),
        axis.z * axis.x * (1.0 - ANGLE.cos()) - axis.y * ANGLE.sin(),
        axis.z * axis.y * (1.0 - ANGLE.cos()) - axis.x * ANGLE.sin(),
        ANGLE.cos() * f64::powi(axis.z, 2) * (1.0 - ANGLE.cos()),
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
    if REP % 2 == 1 {
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
    ]) * rot_mat::<DEG, REP>(axis)
}

pub fn inversion() -> matrix3d::Matrix3d {
    -matrix3d::Matrix3d::new_unit_matrix()
}
