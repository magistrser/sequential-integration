use fehler::throws;

use crate::{
    engine::{helper_equation_traits::EquationOfOneVariable, quadrature::GetQuadratureRange},
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

        let range = G::get_range(a, b, h)?;
        for x in range.iter().take(range.len() - 1) {
            result += equation.calculate(*x, (a, b))?;
        }

        if let Some(last) = range.last() {
            result += equation.calculate_last(*last, (a, b))?;
        }

        result
    }
}
