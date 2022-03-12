mod ops;

#[derive(Clone, Copy, Debug)]
pub struct Matrix3d([f64; 9]);

impl Matrix3d {
    pub fn new(
        x0: f64,
        x1: f64,
        x2: f64,
        x3: f64,
        x4: f64,
        x5: f64,
        x6: f64,
        x7: f64,
        x8: f64,
    ) -> Self {
        Self([x0, x1, x2, x3, x4, x5, x6, x7, x8])
    }

    pub fn new_zero_matrix() -> Matrix3d {
        Self([0.0; 9])
    }

    pub fn new_unit_matrix() -> Matrix3d {
        Self::new(1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0)
    }

    pub fn get_value(&self, i: usize, j: usize) -> f64 {
        self.0[i + 3 * j]
    }

    pub fn set_value(&mut self, i: usize, j: usize, value: f64) {
        self.0[i + 3 * j] = value;
    }
}

impl From<[f64; 9]> for Matrix3d {
    fn from(values: [f64; 9]) -> Matrix3d {
        Self(values)
    }
}
