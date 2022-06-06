use crate::pair_potential::PairPotential;

/// This implements a Lennard-Jones potential with variable exponents
///
/// `eps`: Depth of potential energy well
///
/// `re`: Equilibrium distance
///
/// `n`: Exponent of the repulsive term
///
/// `m`: Exponent of the attractive term
pub struct LennardJonesPotential {
    eps: f64,
    re: f64,
    n: f64,
    m: f64,
}

impl LennardJonesPotential {
    pub fn new(eps: f64, re: f64, n: f64, m: f64) -> Self {
        assert!(
            n > m,
            "Repulsive exponent must be larger than attractive exponent."
        );
        Self { eps, re, n, m }
    }
}

impl PairPotential for LennardJonesPotential {
    fn e(&self, distance: f64) -> f64 {
        self.eps / (self.n / self.m - 1.0)
            * ((f64::powf(self.re / distance, self.n))
                - (self.n / self.m) * (f64::powf(self.re / distance, self.m)))
    }

    fn de_dr(&self, distance: f64) -> f64 {
        -self.eps / (self.re * (self.n / self.m - 1.0))
            * (self.n * (f64::powf(self.re / distance, self.n + 1.0))
                - self.n * (f64::powf(self.re / distance, self.m + 1.0)))
    }

    fn d2e_dr2(&self, distance: f64) -> f64 {
        self.eps / (f64::powi(self.re, 2) * (self.n / self.m - 1.0))
            * ((f64::powi(self.n, 2) + self.n) * f64::powf(self.re / distance, self.n + 2.0)
                - (self.n * self.m + self.n) * f64::powf(self.re / distance, self.m + 2.0))
    }
}

#[cfg(test)]
mod tests {
    use super::LennardJonesPotential;
    use crate::pair_potential::PairPotential;

    fn get_pot() -> LennardJonesPotential {
        LennardJonesPotential::new(1.0, 1.0, 12.0, 6.0)
    }

    #[test]
    fn create_pot() {
        let pot = get_pot();

        assert_eq!(pot.eps, 1.0);
        assert_eq!(pot.re, 1.0);
        assert_eq!(pot.n, 12.0);
        assert_eq!(pot.m, 6.0);
    }

    #[test]
    fn calc_e_value() {
        let pot = get_pot();
        let e0 = pot.e(pot.re);
        let e_long_r = pot.e(pot.re * 2.0);
        let e_short_r = pot.e(pot.re / 2.0);
        let e_lim_r_0 = pot.e(0.0);
        let e_lim_r_inf = pot.e(f64::INFINITY);

        assert_eq!(e0, -pot.eps);
        assert!(e0 < e_long_r);
        assert!(e0 < e_short_r);
        assert!(f64::is_nan(e_lim_r_0));
        assert_eq!(e_lim_r_inf, -0.0);
    }

    #[test]
    fn calc_g_value() {
        let pot = get_pot();
        let g0 = pot.de_dr(pot.re);
        let g_far = pot.de_dr(pot.re * 2.0);
        let g_near = pot.de_dr(pot.re / 2.0);
        let g_lim_r_0 = pot.de_dr(0.0);
        let g_lim_r_inf = pot.de_dr(f64::INFINITY);

        assert_eq!(g0, 0.0);
        assert!(f64::is_sign_positive(g_far));
        assert!(f64::is_sign_negative(g_near));
        assert!(f64::is_nan(g_lim_r_0));
        assert_eq!(g_lim_r_inf, 0.0);
    }

    #[test]
    fn calc_h_value() {
        let pot = get_pot();
        let h0 = pot.d2e_dr2(pot.re);
        let h_far = pot.d2e_dr2(pot.re * 2.0);
        let h_near = pot.d2e_dr2(pot.re / 2.0);
        let h_lim_r_0 = pot.d2e_dr2(0.0);
        let h_lim_r_inf = pot.d2e_dr2(f64::INFINITY);

        assert_eq!(h0, 72.0);
        assert!(f64::is_sign_negative(h_far));
        assert!(f64::is_sign_positive(h_near));
        assert!(f64::is_nan(h_lim_r_0));
        assert_eq!(h_lim_r_inf, 0.0);
    }
}
