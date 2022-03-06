use super::{utils::non_cart_coord, Polyhedron};
use crate::{
    cluster::Cluster,
    spatial::{coord3d::Coord3d, coordinates::Coordinates, matrix3d::Matrix3d, transform3d},
};
use std::f64::consts::PI;

pub fn new(n_layers: &u16) -> Cluster {
    Cluster {
        coords: generate_coords(n_layers),
        kind: Polyhedron::Tetrahedron,
    }
}

fn generate_coords(n_layers: &u16) -> Coordinates {
    let mut transformed = trans_matrix() * non_cart_coord(n_layers);
    let rot_axis_normed = transformed[1] / transformed[1].norm();
    let c5_rot = transform3d::rot_mat::<5, 1>(&rot_axis_normed);
    transformed.add(c5_rot * transformed.clone());

    let inv = transform3d::inversion();
    transformed.add(inv * transformed.clone());

    let rot_axis_normed = transformed[(tet_num(n_layers) - 1) as usize]
        / transformed[(tet_num(n_layers) - 1) as usize].norm();
    let mut temp = Coordinates {
        values: Vec::<Coord3d>::new(),
    };
    let c5 = [
        c5_rot,
        transform3d::rot_mat::<5, 2>(&rot_axis_normed),
        transform3d::rot_mat::<5, 3>(&rot_axis_normed),
        transform3d::rot_mat::<5, 4>(&rot_axis_normed),
        transform3d::rot_mat::<5, 5>(&rot_axis_normed),
    ];

    for i in c5 {
        temp.add(i * transformed.clone());
    }

    transformed.add(temp);
    transformed
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

fn tet_num(n_layers: &u16) -> u16 {
    let mut nt = 0;
    for i in 0..*n_layers {
        nt += (u16::pow(i, 2) + i) / 2;
    }
    nt
}
