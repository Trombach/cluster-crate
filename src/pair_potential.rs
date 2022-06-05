use crate::spatial::{Coord3d, Coordinates};

trait PairPotential {
    fn e(distance: f64) -> f64;
    fn energy(&self, coords: &Coordinates) -> f64 {
        let e: f64 = 0.0;
        for x1 in coords {
            for x2 in coords {
                e += Self::e(x1.dist(&x2));
            }
        }

        e
    }
    fn de_dr(distance: f64) -> f64;
    fn gradient(&self, coords: &Coordinates) -> Vec<Coord3d> {
        let gradient = vec![Coord3d::from([0.0, 0.0, 0.0]); coords.size()];
        for (i, x1) in coords.iter().enumerate() {
            for (j, x2) in coords.iter().enumerate() {
                let d = x1 - x2;
                let g = Self::de_dr(d.norm());
                let gradient_value = d / d.norm() * g;
                gradient[]
            }
        }

        gradient
    }
}
