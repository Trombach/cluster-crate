use crate::{cluster::Coordinates, spatial::coord3d::Coord3d};

pub fn non_cart_coord(n_layers: u16) -> Coordinates {
    let mut vector = Vec::<Coord3d>::new();

    let mut k = 0;
    while k < n_layers {
        let mut l = 0;
        while k + l < n_layers {
            let mut m = 0;
            while k + l + m < n_layers {
                vector.push(Coord3d::from([f64::from(k), f64::from(l), f64::from(m)]));
                m += 1;
            }
            l += 1;
        }
        k += 1;
    }

    vector
}

#[cfg(test)]
mod tests {
    use crate::{cluster::polyhedron::utils, spatial::coord3d::Coord3d};

    #[test]
    fn non_cart_coord() {
        let result = vec![
            Coord3d::from([0.0, 0.0, 0.0]),
            Coord3d::from([0.0, 0.0, 1.0]),
            Coord3d::from([0.0, 0.0, 2.0]),
            Coord3d::from([0.0, 1.0, 0.0]),
            Coord3d::from([0.0, 1.0, 1.0]),
            Coord3d::from([0.0, 2.0, 0.0]),
            Coord3d::from([1.0, 0.0, 0.0]),
            Coord3d::from([1.0, 0.0, 1.0]),
            Coord3d::from([1.0, 1.0, 0.0]),
            Coord3d::from([2.0, 0.0, 0.0]),
        ];
        assert_eq!(utils::non_cart_coord(3), result)
    }
}
