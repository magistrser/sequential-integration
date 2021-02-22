use fehler::throws;

use crate::{engine, errors::Error};

#[throws]
pub fn calculate_double_integral_simpson(
    equation: &str,
    first_integral_begin: f64,
    first_integral_end: f64,
    first_integral_step: f64,
    second_integral_begin: &str,
    second_integral_end: &str,
    second_integral_step: f64,
) -> f64 {
    let simpson_quadrature = engine::quadrature::simpson::SimpsonQuadratureDoubleIntegral::new(
        equation,
        first_integral_step,
        second_integral_step,
    )?;

    engine::calculate_double_integral(
        simpson_quadrature,
        first_integral_begin,
        first_integral_end,
        second_integral_begin,
        second_integral_end,
    )?
}

#[throws]
pub fn calculate_triple_integral_simpson(
    equation: &str,
    first_integral_begin: f64,
    first_integral_end: f64,
    first_integral_step: f64,
    second_integral_begin: &str,
    second_integral_end: &str,
    second_integral_step: f64,
    third_integral_begin: &str,
    third_integral_end: &str,
    third_integral_step: f64,
) -> f64 {
    let simpson_quadrature = engine::quadrature::simpson::SimpsonQuadratureTripleIntegral::new(
        equation,
        first_integral_step,
        second_integral_step,
        third_integral_step,
    )?;

    engine::calculate_triple_integral(
        simpson_quadrature,
        first_integral_begin,
        first_integral_end,
        second_integral_begin,
        second_integral_end,
        third_integral_begin,
        third_integral_end,
    )?
}
