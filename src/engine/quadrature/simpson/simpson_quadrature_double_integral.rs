use fehler::throws;

use super::{simpson_range::SimpsonRangeGenerator, utils as simpson_utils};
use crate::{
    engine::{
        helper_equation_traits::EquationOfTwoVariable,
        quadrature::{
            FinalizeCalculation, GetQuadratureRange, GetStepSizeDoubleIntegral,
            QuadratureDoubleIntegral,
        },
        range_generator::RangeGenerator,
        Bounds, CalculationResult, CalculationStep,
    },
    errors::Error,
};

pub struct SimpsonQuadratureDoubleIntegral<E: Fn(f64, f64) -> f64> {
    equation: E,
    h: f64,
    k: f64,
}

impl<E: Fn(f64, f64) -> f64> SimpsonQuadratureDoubleIntegral<E> {
    #[throws]
    pub fn new(equation: E, h: f64, k: f64) -> Self {
        Self { equation, h, k }
    }

    #[throws]
    fn calculate_simpson(&self, x_values: [f64; 3], y_values: [f64; 3]) -> f64 {
        let mut f = vec![];

        for x in x_values.iter() {
            let mut f_y = vec![];
            for y in y_values.iter() {
                f_y.push((self.equation)(*x, *y));
            }
            f.push(f_y);
        }

        let result = f[0][0]
            + f[2][0]
            + f[0][2]
            + f[2][2]
            + 4. * (f[1][0] + f[0][1] + f[2][1] + f[1][2])
            + 16. * f[1][1];
        result
    }

    fn multiple_with_simpson_constant(value: f64, h: f64, k: f64) -> f64 {
        h * k * value / 9.
    }
}

impl<E: Fn(f64, f64) -> f64> EquationOfTwoVariable for SimpsonQuadratureDoubleIntegral<E> {
    #[throws]
    fn calculate(
        &self,
        x: CalculationStep,
        bounds_x: Bounds,
        y: CalculationStep,
        bounds_y: Bounds,
    ) -> CalculationResult {
        let mut is_last_step = false;

        let x = simpson_utils::SimpsonPoints::generate(x, bounds_x, self.h, &mut is_last_step);
        let y = simpson_utils::SimpsonPoints::generate(y, bounds_y, self.k, &mut is_last_step);

        let x_values = [x.v0, x.v1, x.v2];
        let y_values = [y.v0, y.v1, y.v2];

        let mut result = CalculationResult::new();
        if is_last_step {
            result.add_last(Self::multiple_with_simpson_constant(
                self.calculate_simpson(x_values, y_values)?,
                x.h,
                y.h,
            ));
        } else {
            result.add_common(self.calculate_simpson(x_values, y_values)?);
        }

        result
    }
}

impl<E: Fn(f64, f64) -> f64> FinalizeCalculation for SimpsonQuadratureDoubleIntegral<E> {
    #[throws]
    fn finalize(&self, result: CalculationResult) -> f64 {
        Self::multiple_with_simpson_constant(result.common, self.h, self.k) + result.last
    }
}

impl<E: Fn(f64, f64) -> f64> GetStepSizeDoubleIntegral for SimpsonQuadratureDoubleIntegral<E> {
    fn get_step_size(&self) -> (f64, f64) {
        (self.h, self.k)
    }
}

impl<E: Fn(f64, f64) -> f64> GetQuadratureRange for SimpsonQuadratureDoubleIntegral<E> {
    #[throws]
    fn get_range_generator(bounds: Bounds, h: f64) -> Option<Box<dyn RangeGenerator>> {
        if let Some(range_generator) = SimpsonRangeGenerator::new(bounds, h)? {
            Some(Box::new(range_generator) as Box<dyn RangeGenerator>)
        } else {
            None
        }
    }
}

impl<E: Fn(f64, f64) -> f64> QuadratureDoubleIntegral for SimpsonQuadratureDoubleIntegral<E> {}
