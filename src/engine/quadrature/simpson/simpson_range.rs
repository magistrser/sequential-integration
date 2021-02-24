use fehler::throws;
use snafu::ensure;

use crate::{
    engine::{
        range_generator::{CalculationStep, RangeGenerator},
        Bounds,
    },
    errors::{self, Error},
};

pub struct SimpsonRangeGenerator {
    end: f64,
    h: f64,
    current_step: f64,
}

impl SimpsonRangeGenerator {
    #[throws]
    pub fn new(bounds: Bounds, h: f64) -> Option<Self> {
        if (bounds.end - bounds.begin).abs() == 0. {
            return None;
        }

        Some(Self {
            end: bounds.end,
            h,
            current_step: bounds.begin,
        })
    }
}

impl RangeGenerator for SimpsonRangeGenerator {
    #[throws]
    fn next(&mut self) -> CalculationStep {
        ensure!(
            self.current_step < self.end,
            errors::RangeGeneratorOutOfBounds {
                step: self.current_step,
                end: self.end
            }
        );

        let result = if self.current_step >= self.end - 2. * self.h {
            CalculationStep::Last(self.current_step)
        } else {
            CalculationStep::Common(self.current_step)
        };

        self.current_step += 2. * self.h;
        result
    }
}
