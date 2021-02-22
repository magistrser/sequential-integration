use fehler::throws;
use snafu::ensure;

use crate::{
    engine::range_generator::{RangeGenerator, StepType},
    errors::{self, Error},
};

pub struct SimpsonRangeGenerator {
    a: f64,
    b: f64,
    h: f64,
    current_step: f64,
}

impl SimpsonRangeGenerator {
    #[throws]
    pub fn new(a: f64, b: f64, h: f64) -> Self {
        Self {
            a,
            b,
            h,
            current_step: a,
        }
    }
}

impl RangeGenerator for SimpsonRangeGenerator {
    #[throws]
    fn next(&mut self) -> StepType {
        println!("a: {}, b: {}, current_step: {}", self.a, self.b, self.current_step);
        if (self.a - self.b).abs() == 0. {
            return StepType::NoStep;
        }

        ensure!(
            self.current_step < self.b + self.h,
            errors::RangeGeneratorOutOfBounds {
                step: self.current_step,
                b: self.b
            }
        );

        let result = if self.current_step > self.b - 2. * self.h {
            StepType::Last(self.current_step)
        } else {
            StepType::Common(self.current_step)
        };

        self.current_step += 2. * self.h;
        result
    }
}
