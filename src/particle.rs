use serde::{Deserialize, Serialize};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum SMParticle {
    Electron,
    Neutrino,
    Photon,
}

#[derive(Debug, Copy, Clone, PartialEq, Serialize, Deserialize)]
pub enum BSMParticle {
    ALP(f64, f64),
    DarkPhoton(f64, f64),
}

pub trait DarkMatter {
    fn mass(&self) -> f64;
    fn spin(&self) -> f64;
    fn dof(&self) -> u8;
}

impl DarkMatter for BSMParticle {
    fn mass(&self) -> f64 {
        match self {
            BSMParticle::ALP(mass, _) => *mass,
            BSMParticle::DarkPhoton(mass, _) => *mass,
        }
    }

    fn spin(&self) -> f64 {
        match self {
            BSMParticle::ALP(_, _) => 0f64,
            BSMParticle::DarkPhoton(_, _) => 1f64,
        }
    }

    fn dof(&self) -> u8 {
        match self {
            BSMParticle::ALP(_, _) => 1,
            BSMParticle::DarkPhoton(_, _) => 3,
        }
    }
}
