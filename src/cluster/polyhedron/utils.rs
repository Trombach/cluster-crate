use crate::spatial::{coord3d::Coord3d, coordinates::Coordinates};

pub fn non_cart_coord(n_layers: &u16) -> Coordinates {
    let mut vector = Vec::<Coord3d>::new();

    for k in 0..*n_layers {
        let l = 0;
        while k + l < *n_layers {
            let m = 0;
            while k + l + m < *n_layers {
                vector.push(Coord3d {
                    x: f64::from(k),
                    y: f64::from(l),
                    z: f64::from(m),
                })
            }
        }
    }

    Coordinates::from(vector)
}
