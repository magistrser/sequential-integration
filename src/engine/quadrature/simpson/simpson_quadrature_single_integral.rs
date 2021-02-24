use fehler::throws;

use super::{simpson_range::SimpsonRangeGenerator, utils as simpson_utils};
use crate::{
    engine::{
        helper_equation_traits::EquationOfOneVariable,
        quadrature::{
            FinalizeCalculation, GetQuadratureRange, GetStepSizeSingleIntegral,
            QuadratureSingleIntegral,
        },
        range_generator::RangeGenerator,
        Bounds, CalculationResult, CalculationStep,
    },
    errors::Error,
};

pub struct SimpsonQuadratureSingleIntegral<E: Fn(f64) -> f64> {
    equation: E,
    h: f64,
}

impl<E: Fn(f64) -> f64> SimpsonQuadratureSingleIntegral<E> {
    #[throws]
    pub fn new(equation: E, h: f64) -> Self {
        Self { equation, h }
    }

    #[throws]
    fn calculate_simpson(&self, x_values: [f64; 3]) -> f64 {
        let mut f = vec![];

        for x in x_values.iter() {
            f.push((self.equation)(*x));
        }

        let result = f[0] + 4. * f[1] + f[2];
        result
    }

    fn multiple_with_simpson_constant(value: f64, h: f64) -> f64 {
        h * value / 3.
    }
}

impl<E: Fn(f64) -> f64> EquationOfOneVariable for SimpsonQuadratureSingleIntegral<E> {
    #[throws]
    fn calculate(&self, x: CalculationStep, bounds: Bounds) -> CalculationResult {
        let mut is_last_step = false;
        let x = simpson_utils::SimpsonPoints::generate(x, bounds, self.h, &mut is_last_step);
        let x_values = [x.v0, x.v1, x.v2];

        let mut result = CalculationResult::new();
        if is_last_step {
            result.add_last(Self::multiple_with_simpson_constant(
                self.calculate_simpson(x_values)?,
                x.h,
            ));
        } else {
            result.add_common(self.calculate_simpson(x_values)?);
        }

        result
    }
}

impl<E: Fn(f64) -> f64> FinalizeCalculation for SimpsonQuadratureSingleIntegral<E> {
    #[throws]
    fn finalize(&self, result: CalculationResult) -> f64 {
        Self::multiple_with_simpson_constant(result.common, self.h) + result.last
    }
}

impl<E: Fn(f64) -> f64> GetStepSizeSingleIntegral for SimpsonQuadratureSingleIntegral<E> {
    fn get_step_size(&self) -> f64 {
        self.h
    }
}

impl<E: Fn(f64) -> f64> GetQuadratureRange for SimpsonQuadratureSingleIntegral<E> {
    #[throws]
    fn get_range_generator(bounds: Bounds, h: f64) -> Option<Box<dyn RangeGenerator>> {
        if let Some(range_generator) = SimpsonRangeGenerator::new(bounds, h)? {
            Some(Box::new(range_generator) as Box<dyn RangeGenerator>)
        } else {
            None
        }
    }
}

impl<E: Fn(f64) -> f64> QuadratureSingleIntegral for SimpsonQuadratureSingleIntegral<E> {}
