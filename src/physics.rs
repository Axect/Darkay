use crate::constants::{C, HBARC, T0, TEQ};
use crate::particle::{BSMParticle, SMParticle, DarkMatter};
use peroxide::fuga::*;
use std::f64::consts::PI;

pub fn redshift(t: f64) -> f64 {
    if t < TEQ {
        (T0 / t).sqrt() * (T0 / TEQ).powf(1f64 / 6f64)
    } else {
        (T0 / t).powf(2f64 / 3f64)
    }
}

pub fn inverse_redshift(z: f64) -> f64 {
    if z > (T0 / TEQ).sqrt() {
        T0 / z.powi(2) * (T0 / TEQ).powf(1f64 / 3f64)
    } else {
        T0 / z.powf(3f64 / 2f64)
    }
}

#[allow(non_snake_case)]
pub fn Z(t_vec: &Vec<f64>) -> Vec<f64> {
    t_vec.fmap(redshift)
}

// ┌─────────────────────────────────────────────────────────┐
//  Decay (Traditional and Time-Varying)
// └─────────────────────────────────────────────────────────┘
#[allow(non_snake_case)]
pub trait Decay {
    fn decay_rate(&self, to: SMParticle) -> f64;
    fn life_time(&self, to: SMParticle) -> f64 {
        1f64 / self.decay_rate(to)
    }
    fn P_surv(&self, to: SMParticle, t: f64, t_e: f64) -> f64 {
        (-self.decay_rate(to) * (t - t_e)).exp()
    }
    fn P_decay(&self, to: SMParticle, t: f64, t_e: f64) -> f64 {
        self.decay_rate(to) * self.P_surv(to, t, t_e)
    }
}

impl Decay for BSMParticle {
    fn decay_rate(&self, to: SMParticle) -> f64 {
        match self {
            BSMParticle::ALP(m, g) => match to {
                SMParticle::Photon => {
                    g.powi(2) * m.powi(3) / (64f64 * PI * 1e+6 * HBARC / C * 1e-13)
                }
                _ => unimplemented!(),
            },
            BSMParticle::DarkPhoton(_, _) => unimplemented!(),
        }
    }
}

#[allow(non_snake_case)]
pub trait TimeVaryingDecay: Decay + DarkMatter {
    fn energy_tv(&self, t: f64, t_e: f64, E: f64) -> f64 {
        let mass = self.mass();
        if mass == 0f64 {
            E * redshift(t) / redshift(t_e)
        } else {
            (mass.powi(2) + (E.powi(2) - mass.powi(2)) * (redshift(t) / redshift(t_e)).powi(2)).sqrt()
        }
    }

    fn lorentz_factor(&self, t: f64, t_e: f64, E: f64) -> f64 {
        self.energy_tv(t, t_e, E) / self.mass()
    }

    fn decay_rate_tv(&self, to: SMParticle, t: f64, t_e: f64, E: f64) -> f64 {
        self.decay_rate(to) / self.lorentz_factor(t, t_e, E)
    }

    fn life_time_tv(&self, to: SMParticle, t: f64, t_e: f64, E: f64) -> f64 {
        self.life_time(to) * self.lorentz_factor(t, t_e, E)
    }

    fn P_surv_tv(&self, to: SMParticle, t: f64, t_e: f64, E: f64) -> f64 {
        // Change variable : t' -> t' / t
        let f = |t_prime: f64| {
            // Recover t_prime
            let t_prime = t_prime * t;
            self.decay_rate_tv(to, t_prime, t_e, E)
        };

        let integral = integrate(f, (t_e / t, 1f64), G7K15R(1e-4, 20)) * t;
        (-integral).exp()
    }

    fn P_decay_tv(&self, to: SMParticle, t: f64, t_e: f64, E: f64) -> f64 {
        self.decay_rate_tv(to, t, t_e, E) * self.P_surv_tv(to, t, t_e, E)
    }
}

impl TimeVaryingDecay for BSMParticle {}
