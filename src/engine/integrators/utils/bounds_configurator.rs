use fehler::throws;

use crate::{engine::Bounds, errors::Error};

pub struct BoundsConfigurator {
    pub bounds: Bounds,
    pub direction_coeff: f64,
}

impl BoundsConfigurator {
    #[throws]
    pub fn configurate(a: f64, b: f64) -> Self {
        let begin = a.min(b);
        let end = a.max(b);
        let direction_coeff = if begin != a { -1. } else { 1. };

        Self {
            bounds: Bounds::new(begin, end)?,
            direction_coeff,
        }
    }
}
