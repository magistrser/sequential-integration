use fehler::throws;

use crate::{
    engine::{
        helper_equation_traits::EquationOfOneVariable, quadrature::GetQuadratureRange,
        range_generator::StepType,
    },
    errors::Error,
};

pub struct FirstIntegrator;

impl FirstIntegrator {
    #[throws]
    pub fn integrate<E: EquationOfOneVariable, G: GetQuadratureRange>(
        a: f64,
        b: f64,
        h: f64,
        equation: E,
    ) -> f64 {
        let mut result = 0.;
        let mut range = G::get_range_generator(a, b, h)?;
        loop {
            match range.next()? {
                StepType::Common(x) => result += equation.calculate(x, (a, b))?,
                StepType::Last(x) => {
                    result += equation.calculate_last(x, (a, b))?;
                    break;
                }
                StepType::NoStep => break,
            }
        }

        result
    }
}
