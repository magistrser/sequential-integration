use fehler::throws;
use snafu::ensure;

use crate::{
    engine::range_generator::{CalculationStep, RangeGenerator},
    errors::{self, Error},
};

pub struct SimpsonRangeGenerator {
    b: f64,
    h: f64,
    current_step: f64,
}

impl SimpsonRangeGenerator {
    #[throws]
    pub fn new(a: f64, b: f64, h: f64) -> Option<Self> {
        if (a - b).abs() == 0. {
            return None;
        }

        Some(Self {
            b,
            h,
            current_step: a,
        })
    }
}

impl RangeGenerator for SimpsonRangeGenerator {
    #[throws]
    fn next(&mut self) -> CalculationStep {
        ensure!(
            self.current_step < self.b,
            errors::RangeGeneratorOutOfBounds {
                step: self.current_step,
                b: self.b
            }
        );

        let result = if self.current_step >= self.b - 2. * self.h {
            CalculationStep::Last(self.current_step)
        } else {
            CalculationStep::Common(self.current_step)
        };

        self.current_step += 2. * self.h;
        result
    }
}
