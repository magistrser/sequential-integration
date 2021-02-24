mod calculation_step;
pub mod helper_equation_traits;
mod integrators;
pub mod quadrature;
pub mod range_generator;
pub use calculation_step::CalculationStep;
mod calculation_result;
pub use calculation_result::CalculationResult;
mod bounds;
pub use bounds::Bounds;
pub mod function_types;

use fehler::throws;

use crate::errors::Error;

#[throws]
pub fn calculate_single_integral<Q: quadrature::QuadratureSingleIntegral>(
    quadrature: Q,
    first_integral_begin: f64,
    first_integral_end: f64,
) -> f64 {
    let result = integrators::Integrator::integrate::<Q, Q>(
        first_integral_begin,
        first_integral_end,
        quadrature.get_step_size(),
        &quadrature,
        &quadrature,
    )?;

    result
}

#[throws]
pub fn calculate_double_integral<
    Q: quadrature::QuadratureDoubleIntegral,
    F1: Fn(f64) -> f64,
    F2: Fn(f64) -> f64,
>(
    quadrature: Q,
    first_integral_begin: f64,
    first_integral_end: f64,
    second_integral_begin: F1,
    second_integral_end: F2,
) -> f64 {
    let second_integrator = integrators::SecondIntegrator::<Q, Q, F1, F2>::new(
        second_integral_begin,
        second_integral_end,
        quadrature.get_step_size().1,
        &quadrature,
    )?;

    let result =
        integrators::Integrator::integrate::<integrators::SecondIntegrator<Q, Q, F1, F2>, Q>(
            first_integral_begin,
            first_integral_end,
            quadrature.get_step_size().0,
            &second_integrator,
            &quadrature,
        )?;

    result
}

#[throws]
pub fn calculate_triple_integral<
    Q: quadrature::QuadratureTripleIntegral,
    F1: Fn(f64) -> f64,
    F2: Fn(f64) -> f64,
    F3: Fn(f64, f64) -> f64,
    F4: Fn(f64, f64) -> f64,
>(
    quadrature: Q,
    first_integral_begin: f64,
    first_integral_end: f64,
    second_integral_begin: F1,
    second_integral_end: F2,
    third_integral_begin: F3,
    third_integral_end: F4,
) -> f64 {
    let third_integrator = integrators::ThirdIntegrator::<Q, Q, F3, F4>::new(
        third_integral_begin,
        third_integral_end,
        quadrature.get_step_size().2,
        &quadrature,
    )?;

    let second_integrator = integrators::SecondIntegrator::<
        Q,
        integrators::ThirdIntegrator<Q, Q, F3, F4>,
        F1,
        F2,
    >::new(
        second_integral_begin,
        second_integral_end,
        quadrature.get_step_size().1,
        &third_integrator,
    )?;

    let result = integrators::Integrator::integrate::<
        integrators::SecondIntegrator<Q, integrators::ThirdIntegrator<Q, Q, F3, F4>, F1, F2>,
        Q,
    >(
        first_integral_begin,
        first_integral_end,
        quadrature.get_step_size().0,
        &second_integrator,
        &quadrature,
    )?;

    result
}
