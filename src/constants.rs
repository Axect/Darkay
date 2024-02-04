use natural_unit::CONSTANT_CGS;

pub const MPBH: f64 = 1e+15;                    // g
pub const RHODM: f64 = 2.35e-30;                // g/cm^3
pub const RHO0: f64 = 0.4;                      // GeV/cm^3
pub const FPBH: f64 = 1f64;
pub const C: f64 = CONSTANT_CGS.c;              // cm/s
pub const HBARC: f64 = 197.3269804;             // MeV*fm
pub const YEAR: f64 = 365.24 * 24f64 * 3600f64; // s
pub const T0: f64 = 13.8e+9 * YEAR;             // s
pub const TEQ: f64 = 1.48e+12;                  // s
pub const TCMB: f64 = 3.8e+5 * YEAR;            // s
pub const ERG2MEV: f64 = 624151f64;             // MeV/erg
pub const RH: f64 = 200f64;                     // kpc
pub const RS: f64 = 20f64;                      // kpc
pub const RSUN: f64 = 8.5f64;                   // kpc
pub const KPC2CM: f64 = 3.08567758149137e+21;   // cm/kpc

pub const NDEC: usize = 100;

pub const ME: f64 = 0.5109989461;               // MeV
pub const RE: f64 = 2.8179403227e-13;           // cm
