use fehler::throws;

use super::{simpson_range::SimpsonRangeGenerator, utils as simpson_utils};
use crate::{
    engine::{
        helper_equation_traits::EquationOfThreeVariable,
        quadrature::{
            FinalizeCalculation, GetQuadratureRange, GetStepSizeTripleIntegral,
            QuadratureTripleIntegral,
        },
        range_generator::RangeGenerator,
        Bounds, CalculationResult, CalculationStep,
    },
    errors::Error,
};

pub struct SimpsonQuadratureTripleIntegral<E: Fn(f64, f64, f64) -> f64> {
    equation: E,
    h: f64,
    k: f64,
    l: f64,
}

impl<E: Fn(f64, f64, f64) -> f64> SimpsonQuadratureTripleIntegral<E> {
    #[throws]
    pub fn new(equation: E, h: f64, k: f64, l: f64) -> Self {
        Self { equation, h, k, l }
    }

    #[throws]
    fn calculate_simpson(&self, x_values: [f64; 3], y_values: [f64; 3], z_values: [f64; 3]) -> f64 {
        let mut f = vec![];

        for x in x_values.iter() {
            let mut f_y = vec![];
            for y in y_values.iter() {
                let mut f_z = vec![];
                for z in z_values.iter() {
                    f_z.push((self.equation)(*x, *y, *z));
                }
                f_y.push(f_z);
            }
            f.push(f_y);
        }

        let result = f[0][0][0]
            + 4. * f[1][0][0]
            + f[2][0][0]
            + 4. * (f[0][1][0] + 4. * f[1][1][0] + f[2][1][0])
            + f[0][2][0]
            + 4. * f[1][2][0]
            + f[2][2][0]
            + 4. * (f[0][0][1]
                + 4. * f[1][0][1]
                + f[2][0][1]
                + 4. * (f[0][1][1] + 4. * f[1][1][1] + f[2][1][1])
                + f[0][2][1]
                + 4. * f[1][2][1]
                + f[2][2][1])
            + f[0][0][2]
            + 4. * f[1][0][2]
            + f[2][0][2]
            + 4. * (f[0][1][2] + 4. * f[1][1][2] + f[2][1][2])
            + f[0][2][2]
            + 4. * f[1][2][2]
            + f[2][2][2];
        result
    }

    fn multiple_with_simpson_constant(value: f64, h: f64, k: f64, l: f64) -> f64 {
        h * k * l * value / 27.
    }
}

impl<E: Fn(f64, f64, f64) -> f64> EquationOfThreeVariable for SimpsonQuadratureTripleIntegral<E> {
    #[throws]
    fn calculate(
        &self,
        x: CalculationStep,
        bounds_x: Bounds,
        y: CalculationStep,
        bounds_y: Bounds,
        z: CalculationStep,
        bounds_z: Bounds,
    ) -> CalculationResult {
        let mut is_last_step = false;

        let x = simpson_utils::SimpsonPoints::generate(x, bounds_x, self.h, &mut is_last_step);
        let y = simpson_utils::SimpsonPoints::generate(y, bounds_y, self.k, &mut is_last_step);
        let z = simpson_utils::SimpsonPoints::generate(z, bounds_z, self.l, &mut is_last_step);

        let x_values = [x.v0, x.v1, x.v2];
        let y_values = [y.v0, y.v1, y.v2];
        let z_values = [z.v0, z.v1, z.v2];

        let mut result = CalculationResult::new();
        if is_last_step {
            result.add_last(Self::multiple_with_simpson_constant(
                self.calculate_simpson(x_values, y_values, z_values)?,
                x.h,
                y.h,
                z.h,
            ));
        } else {
            result.add_common(self.calculate_simpson(x_values, y_values, z_values)?);
        }

        result
    }
}

impl<E: Fn(f64, f64, f64) -> f64> FinalizeCalculation for SimpsonQuadratureTripleIntegral<E> {
    #[throws]
    fn finalize(&self, result: CalculationResult) -> f64 {
        Self::multiple_with_simpson_constant(result.common, self.h, self.k, self.l) + result.last
    }
}

impl<E: Fn(f64, f64, f64) -> f64> GetStepSizeTripleIntegral for SimpsonQuadratureTripleIntegral<E> {
    fn get_step_size(&self) -> (f64, f64, f64) {
        (self.h, self.k, self.l)
    }
}

impl<E: Fn(f64, f64, f64) -> f64> GetQuadratureRange for SimpsonQuadratureTripleIntegral<E> {
    #[throws]
    fn get_range_generator(bounds: Bounds, h: f64) -> Option<Box<dyn RangeGenerator>> {
        if let Some(range_generator) = SimpsonRangeGenerator::new(bounds, h)? {
            Some(Box::new(range_generator) as Box<dyn RangeGenerator>)
        } else {
            None
        }
    }
}

impl<E: Fn(f64, f64, f64) -> f64> QuadratureTripleIntegral for SimpsonQuadratureTripleIntegral<E> {}
