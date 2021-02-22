use core::marker::PhantomData;
use fehler::throws;
use mexprp::{Context, Expression};

use crate::{
    engine::{
        helper_equation_traits::{Bounds, EquationOfThreeVariable, EquationOfTwoVariable},
        quadrature::GetQuadratureRange,
        range_generator::StepType,
        utils,
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
    fn calculate(&self, x: f64, bounds_x: Bounds, y: f64, bounds_y: Bounds) -> f64 {
        let mut context = Context::new();
        context.set_var("x", x);
        context.set_var("y", y);

        let a = utils::calculate_expression_one_value_result(&context, &self.a_equation)?;
        let b = utils::calculate_expression_one_value_result(&context, &self.b_equation)?;

        let mut result = 0.;
        let mut range = G::get_range_generator(a, b, self.h)?;
        loop {
            match range.next()? {
                StepType::Common(z) => {
                    result += self
                        .equation
                        .calculate(x, bounds_x, y, bounds_y, z, (a, b))?
                }
                StepType::Last(z) => {
                    result += self
                        .equation
                        .calculate_last(x, bounds_x, y, bounds_y, z, (a, b))?;
                    break;
                }
                StepType::NoStep => break,
            }
        }

        result
    }

    #[throws]
    fn calculate_last(&self, x: f64, bounds_x: Bounds, y: f64, bounds_y: Bounds) -> f64 {
        self.calculate(x, bounds_x, y, bounds_y)?
    }
}
