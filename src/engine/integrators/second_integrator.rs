use core::marker::PhantomData;
use fehler::throws;
use mexprp::{Context, Expression};

use crate::{
    engine::{
        helper_equation_traits::{Bounds, EquationOfOneVariable, EquationOfTwoVariable},
        quadrature::GetQuadratureRange,
        range_generator::StepType,
        utils,
    },
    errors::Error,
};

pub struct SecondIntegrator<G: GetQuadratureRange, E: EquationOfTwoVariable> {
    a_equation: Expression<f64>,
    b_equation: Expression<f64>,
    h: f64,
    equation: E,
    _p: PhantomData<G>,
}

impl<G: GetQuadratureRange, E: EquationOfTwoVariable> SecondIntegrator<G, E> {
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

impl<G: GetQuadratureRange, E: EquationOfTwoVariable> EquationOfOneVariable
    for SecondIntegrator<G, E>
{
    #[throws]
    fn calculate(&self, x: f64, bounds: Bounds) -> f64 {
        let mut context = Context::new();
        context.set_var("x", x);

        let a = utils::calculate_expression_one_value_result(&context, &self.a_equation)?;
        let b = utils::calculate_expression_one_value_result(&context, &self.b_equation)?;

        let mut result = 0.;
        let mut range = G::get_range_generator(a, b, self.h)?;
        loop {
            match range.next()? {
                StepType::Common(y) => result += self.equation.calculate(x, bounds, y, (a, b))?,
                StepType::Last(y) => {
                    result += self.equation.calculate_last(x, bounds, y, (a, b))?;
                    break;
                }
                StepType::NoStep => break,
            }
        }

        result
    }

    #[throws]
    fn calculate_last(&self, x: f64, bounds: Bounds) -> f64 {
        self.calculate(x, bounds)?
    }
}
