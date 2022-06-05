use super::Cluster;
use crate::spatial::Coord3d;
use argmin::prelude::*;

impl ArgminOp for Cluster {
    type Param = Vec<Coord3d>;

    type Output = f64;

    type Hessian = Vec<Vec<Coord3d>>;

    type Jacobian = ();

    type Float = f64;

    fn apply(&self, _param: &Self::Param) -> Result<Self::Output, Error> {}
}
