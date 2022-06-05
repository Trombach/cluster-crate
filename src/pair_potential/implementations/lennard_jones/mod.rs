use crate::pair_potential::PairPotential;

pub struct LennardJonesPotential {
    epsilon: f64,
    rm: f64,
    exp1: f64,
    exp2: f64,
}

impl LennardJonesPotential {
    pub fn new(epsilon: f64, rm: f64, exp1: f64, exp2: f64) -> Self {
        Self {
            epsilon,
            rm,
            exp1,
            exp2,
        }
    }
}

impl PairPotential for LennardJonesPotential {
    fn e(&self, distance: f64) -> f64 {
        self.epsilon / (self.exp1 / self.exp2 - 1.0)
            * ((f64::powf(self.rm / distance, self.exp1))
                - (self.exp1 / self.exp2) * (f64::powf(self.rm / distance, self.exp2)))
    }

    fn de_dr(&self, distance: f64) -> f64 {
        -self.epsilon / (self.rm * (self.exp1 / self.exp2 - 1.0))
            * (self.exp1 * (f64::powf(self.rm / distance, self.exp1 + 1.0))
                - self.exp1 * (f64::powf(self.rm / distance, self.exp2 + 1.0)))
    }

    fn d2e_dr2(&self, distance: f64) -> f64 {
        self.epsilon / (f64::powi(self.rm, 2) * (self.exp1 / self.exp2 - 1.0))
            * ((f64::powi(self.exp1, 2) + self.exp1)
                * f64::powf(self.rm / distance, self.exp1 + 2.0)
                - (self.exp1 * self.exp2 + self.exp1)
                    * f64::powf(self.rm / distance, self.exp2 + 2.0))
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

        println!("{}", (12.0 / 6.0 - 1.0));

        assert_eq!(pot.epsilon, 1.0);
        assert_eq!(pot.rm, 1.0);
        assert_eq!(pot.exp1, 12.0);
        assert_eq!(pot.exp2, 6.0);
    }

    #[test]
    fn calc_e_value() {
        let pot = get_pot();
        let e0 = pot.e(pot.rm);
        let e_long_r = pot.e(pot.rm * 2.0);
        let e_short_r = pot.e(pot.rm / 2.0);
        let e_lim_r_0 = pot.e(0.0);
        let e_lim_r_inf = pot.e(f64::INFINITY);

        assert_eq!(e0, -1.0);
        assert!(e0 < e_long_r);
        assert!(e0 < e_short_r);
        assert!(f64::is_nan(e_lim_r_0));
        assert_eq!(e_lim_r_inf, -0.0);
    }

    #[test]
    fn calc_g_value() {
        let pot = get_pot();
        let g0 = pot.de_dr(pot.rm);
        let g_far = pot.de_dr(pot.rm * 2.0);
        let g_near = pot.de_dr(pot.rm / 2.0);
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
        let h0 = pot.d2e_dr2(pot.rm);

        println!("{}", h0);
    }
}
