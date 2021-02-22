mod helper_equation_traits;
mod integrators;
pub mod quadrature;
mod utils;

use fehler::throws;

use crate::errors::Error;

#[throws]
pub fn calculate_double_integral<Q: quadrature::QuadratureDoubleIntegral>(
    quadrature: Q,
    first_integral_begin: f64,
    first_integral_end: f64,
    second_integral_begin: &str,
    second_integral_end: &str,
) -> f64 {
    let second_integrator = integrators::SecondIntegrator::<Q, Q>::new(
        second_integral_begin,
        second_integral_end,
        quadrature.get_step_size().1,
        quadrature.clone(),
    )?;

    let not_finilized_result =
        integrators::Integrator::integrate::<integrators::SecondIntegrator<Q, Q>, Q>(
            first_integral_begin,
            first_integral_end,
            quadrature.get_step_size().0,
            second_integrator,
        )?;

    let result = quadrature.finalize(not_finilized_result)?;
    result
}

#[throws]
pub fn calculate_triple_integral<Q: quadrature::QuadratureTripleIntegral>(
    quadrature: Q,
    first_integral_begin: f64,
    first_integral_end: f64,
    second_integral_begin: &str,
    second_integral_end: &str,
    third_integral_begin: &str,
    third_integral_end: &str,
) -> f64 {
    let third_integrator = integrators::ThirdIntegrator::<Q, Q>::new(
        third_integral_begin,
        third_integral_end,
        quadrature.get_step_size().2,
        quadrature.clone(),
    )?;

    let second_integrator =
        integrators::SecondIntegrator::<Q, integrators::ThirdIntegrator<Q, Q>>::new(
            second_integral_begin,
            second_integral_end,
            quadrature.get_step_size().1,
            third_integrator,
        )?;

    let not_finilized_result = integrators::Integrator::integrate::<
        integrators::SecondIntegrator<Q, integrators::ThirdIntegrator<Q, Q>>,
        Q,
    >(
        first_integral_begin,
        first_integral_end,
        quadrature.get_step_size().0,
        second_integrator,
    )?;

    let result = quadrature.finalize(not_finilized_result)?;
    result
}
