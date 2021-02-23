use fehler::throws;

use crate::{
    engine::{
        helper_equation_traits::EquationOfOneVariable,
        quadrature::{FinalizeCalculation, GetQuadratureRange},
        CalculationResult,
    },
    errors::Error,
};

pub struct FirstIntegrator;

impl FirstIntegrator {
    #[throws]
    pub fn integrate<E: EquationOfOneVariable, G: GetQuadratureRange + FinalizeCalculation>(
        a: f64,
        b: f64,
        h: f64,
        equation: E,
        quadrature: G,
    ) -> f64 {
        let mut result = CalculationResult::new();
        let mut range = if let Some(range) = G::get_range_generator(a, b, h)? {
            range
        } else {
            return result.common;
        };

        loop {
            let step = range.next()?;
            result += equation.calculate(step, (a, b))?;

            if step.is_last() {
                break;
            }
        }

        quadrature.finalize(result)?
    }
}
