use core::marker::PhantomData;
use fehler::throws;

use super::utils as integrator_utils;
use crate::{
    engine::{
        helper_equation_traits::{EquationOfOneVariable, EquationOfTwoVariable},
        quadrature::GetQuadratureRange,
        Bounds, CalculationResult, CalculationStep,
    },
    errors::Error,
};

pub struct SecondIntegrator<
    'a,
    G: GetQuadratureRange,
    E: EquationOfTwoVariable,
    F1: Fn(f64) -> f64,
    F2: Fn(f64) -> f64,
> {
    a_equation: F1,
    b_equation: F2,
    h: f64,
    equation: &'a E,
    _p: PhantomData<G>,
}

impl<
        'a,
        G: GetQuadratureRange,
        E: EquationOfTwoVariable,
        F1: Fn(f64) -> f64,
        F2: Fn(f64) -> f64,
    > SecondIntegrator<'a, G, E, F1, F2>
{
    #[throws]
    pub fn new(a_equation: F1, b_equation: F2, h: f64, equation: &'a E) -> Self {
        Self {
            a_equation,
            b_equation,
            h,
            equation,
            _p: PhantomData,
        }
    }
}

impl<
        'a,
        G: GetQuadratureRange,
        E: EquationOfTwoVariable,
        F1: Fn(f64) -> f64,
        F2: Fn(f64) -> f64,
    > EquationOfOneVariable for SecondIntegrator<'a, G, E, F1, F2>
{
    #[throws]
    fn calculate(&self, x: CalculationStep, bounds: Bounds) -> CalculationResult {
        let a = (self.a_equation)(*x);
        let b = (self.b_equation)(*x);
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
            result += self
                .equation
                .calculate(x, bounds, step, borders_config.bounds)?
                * borders_config.direction_coeff;

            if step.is_last() {
                break;
            }
        }

        result
    }
}
