use core::marker::PhantomData;
use fehler::throws;
use mexprp::{Context, Expression};

use super::utils as integrator_utils;
use crate::{
    engine::{
        helper_equation_traits::{EquationOfThreeVariable, EquationOfTwoVariable},
        quadrature::GetQuadratureRange,
        utils, Bounds, CalculationResult, CalculationStep,
    },
    errors::Error,
};

pub struct ThirdIntegrator<G: GetQuadratureRange, E: EquationOfThreeVariable> {
    a_equation: Expression<f64>,
    b_equation: Expression<f64>,
    h: f64,
    equation: E,
    _p: PhantomData<G>,
}

impl<G: GetQuadratureRange, E: EquationOfThreeVariable> ThirdIntegrator<G, E> {
    #[throws]
    pub fn new(a_equation: &str, b_equation: &str, h: f64, equation: E) -> Self {
        let a_equation = Expression::parse(a_equation)?;
        let b_equation = Expression::parse(b_equation)?;

        Self {
            a_equation,
            b_equation,
            h,
            equation,
            _p: PhantomData,
        }
    }
}

impl<G: GetQuadratureRange, E: EquationOfThreeVariable> EquationOfTwoVariable
    for ThirdIntegrator<G, E>
{
    #[throws]
    fn calculate(
        &self,
        x: CalculationStep,
        bounds_x: Bounds,
        y: CalculationStep,
        bounds_y: Bounds,
    ) -> CalculationResult {
        let mut context = Context::new();
        context.set_var("x", *x);
        context.set_var("y", *y);

        let a = utils::calculate_expression_one_value_result(&context, &self.a_equation)?;
        let b = utils::calculate_expression_one_value_result(&context, &self.b_equation)?;
        let borders_config = integrator_utils::BoundsConfigurator::configurate(a, b)?;

        let mut result = CalculationResult::new();
        let mut range = if let Some(range) = G::get_range_generator(borders_config.bounds, self.h)?
        {
            range
        } else {
            return result;
        };

        loop {
            let step = range.next()?;
            result +=
                self.equation
                    .calculate(x, bounds_x, y, bounds_y, step, borders_config.bounds)?
                    * borders_config.direction_coeff;

            if step.is_last() {
                break;
            }
        }

        result
    }
}
