pub mod implementations;

use crate::{cluster::Coordinates, spatial::Coord3d};

trait PairPotential {
    fn e(&self, distance: f64) -> f64;
    fn energy(&self, coords: &Coordinates) -> f64 {
        let mut e: f64 = 0.0;
        for x1 in coords {
            for x2 in coords {
                e += self.e(x1.dist(&x2));
            }
        }

        e
    }
    fn de_dr(&self, distance: f64) -> f64;
    fn gradient(&self, coords: &Coordinates) -> Vec<Coord3d> {
        let mut gradient = vec![Coord3d::from([0.0, 0.0, 0.0]); coords.len()];
        for (i, x1) in coords.iter().enumerate() {
            for (j, x2) in coords.iter().enumerate() {
                let d = *x1 - *x2;
                let g = self.de_dr(d.norm());
                let gradient_value = d / d.norm() * g;
                gradient[i] += gradient_value;
                gradient[j] -= gradient_value;
            }
        }

        gradient
    }
    fn d2e_dr2(&self, distance: f64) -> f64;
    fn hessian(&self, coords: &Coordinates) -> Vec<Vec<f64>> {
        let mut hessian = vec![vec![0f64; 3 * coords.len()]; 3 * coords.len()];
        for (i, x1) in coords.iter().enumerate() {
            for (j, x2) in coords.iter().enumerate() {
                let d = *x1 - *x2;
                let r = d.norm();

                // first and second derivative values
                let de_dr = self.de_dr(r);
                let d2e_dr2 = self.d2e_dr2(r);

                // derivatives of r
                let dvecr_dr = d.d_norm();
                let d2rvecr_dr2 = d.dd_norm();

                // calculate hessian elements by looping over all 6 coordinates of 1 pair of coordinates
                for k in 0..2 {
                    for l in 0..2 {
                        let hessian_value = de_dr * d2rvecr_dr2.get_value(l, k)
                            + d2e_dr2 * dvecr_dr[k] * dvecr_dr[l];

                        hessian[3 * i + k][3 * i + l] += hessian_value;
                        hessian[3 * i + k][3 * j + l] -= hessian_value;
                        hessian[3 * j + k][3 * i + l] -= hessian_value;
                        hessian[3 * j + k][3 * j + l] += hessian_value;
                    }
                }
            }
        }
        hessian
    }
}
