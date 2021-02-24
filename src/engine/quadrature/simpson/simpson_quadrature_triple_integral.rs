use fehler::throws;
use mexprp::{Context, Expression};

use super::{simpson_range::SimpsonRangeGenerator, utils as simpson_utils};
use crate::{
    engine::{
        helper_equation_traits::EquationOfThreeVariable,
        quadrature::{
            FinalizeCalculation, GetQuadratureRange, GetStepSizeTripleIntegral,
            QuadratureTripleIntegral,
        },
        range_generator::RangeGenerator,
        utils, Bounds, CalculationResult, CalculationStep,
    },
    errors::Error,
};

pub struct SimpsonQuadratureTripleIntegral {
    equation: Expression<f64>,
    h: f64,
    k: f64,
    l: f64,
}

impl Clone for SimpsonQuadratureTripleIntegral {
    fn clone(&self) -> Self {
        Self::new(self.equation.string.as_str(), self.h, self.k, self.l).unwrap()
    }
}

impl SimpsonQuadratureTripleIntegral {
    #[throws]
    pub fn new(equation: &str, h: f64, k: f64, l: f64) -> Self {
        let equation = Expression::parse(equation)?;
        Self { equation, h, k, l }
    }

    #[throws]
    fn calculate_simpson(&self, x_values: [f64; 3], y_values: [f64; 3], z_values: [f64; 3]) -> f64 {
        let mut context = Context::new();
        let mut f = vec![];

        for x in x_values.iter() {
            let mut f_y = vec![];
            for y in y_values.iter() {
                let mut f_z = vec![];
                for z in z_values.iter() {
                    context.set_var("x", *x);
                    context.set_var("y", *y);
                    context.set_var("z", *z);

                    f_z.push(utils::calculate_expression_one_value_result(
                        &context,
                        &self.equation,
                    )?);
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

impl EquationOfThreeVariable for SimpsonQuadratureTripleIntegral {
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

impl FinalizeCalculation for SimpsonQuadratureTripleIntegral {
    #[throws]
    fn finalize(&self, result: CalculationResult) -> f64 {
        Self::multiple_with_simpson_constant(result.common, self.h, self.k, self.l) + result.last
    }
}

impl GetStepSizeTripleIntegral for SimpsonQuadratureTripleIntegral {
    fn get_step_size(&self) -> (f64, f64, f64) {
        (self.h, self.k, self.l)
    }
}

impl GetQuadratureRange for SimpsonQuadratureTripleIntegral {
    #[throws]
    fn get_range_generator(bounds: Bounds, h: f64) -> Option<Box<dyn RangeGenerator>> {
        if let Some(range_generator) = SimpsonRangeGenerator::new(bounds, h)? {
            Some(Box::new(range_generator) as Box<dyn RangeGenerator>)
        } else {
            None
        }
    }
}

impl QuadratureTripleIntegral for SimpsonQuadratureTripleIntegral {}
