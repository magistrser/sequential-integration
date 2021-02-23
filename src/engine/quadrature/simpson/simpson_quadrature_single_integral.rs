use fehler::throws;
use mexprp::{Context, Expression};

use super::{simpson_range::SimpsonRangeGenerator, utils as simpson_utils};
use crate::{
    engine::{
        helper_equation_traits::{Bounds, EquationOfOneVariable},
        quadrature::{
            FinalizeCalculation, GetQuadratureRange, GetStepSizeSingleIntegral,
            QuadratureSingleIntegral,
        },
        range_generator::RangeGenerator,
        utils, CalculationResult, CalculationStep,
    },
    errors::Error,
};

pub struct SimpsonQuadratureSingleIntegral {
    equation: Expression<f64>,
    h: f64,
}

impl Clone for SimpsonQuadratureSingleIntegral {
    fn clone(&self) -> Self {
        Self::new(self.equation.string.as_str(), self.h).unwrap()
    }
}

impl SimpsonQuadratureSingleIntegral {
    #[throws]
    pub fn new(equation: &str, h: f64) -> Self {
        let equation = Expression::parse(equation)?;
        Self { equation, h }
    }

    #[throws]
    fn calculate_simpson(&self, x_values: [f64; 3]) -> f64 {
        let mut context = Context::new();
        let mut f = vec![];

        for x in x_values.iter() {
            context.set_var("x", *x);
            f.push(utils::calculate_expression_one_value_result(
                &context,
                &self.equation,
            )?);
        }

        let result = f[0] + 4. * f[1] + f[2];
        result
    }

    fn multiple_with_simpson_constant(value: f64, h: f64) -> f64 {
        h * value / 3.
    }
}

impl EquationOfOneVariable for SimpsonQuadratureSingleIntegral {
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

impl FinalizeCalculation for SimpsonQuadratureSingleIntegral {
    #[throws]
    fn finalize(&self, result: CalculationResult) -> f64 {
        Self::multiple_with_simpson_constant(result.common, self.h) + result.last
    }
}

impl GetStepSizeSingleIntegral for SimpsonQuadratureSingleIntegral {
    fn get_step_size(&self) -> f64 {
        self.h
    }
}

impl GetQuadratureRange for SimpsonQuadratureSingleIntegral {
    #[throws]
    fn get_range_generator(a: f64, b: f64, h: f64) -> Option<Box<dyn RangeGenerator>> {
        if let Some(range_generator) = SimpsonRangeGenerator::new(a, b, h)? {
            Some(Box::new(range_generator) as Box<dyn RangeGenerator>)
        } else {
            None
        }
    }
}

impl QuadratureSingleIntegral for SimpsonQuadratureSingleIntegral {}
