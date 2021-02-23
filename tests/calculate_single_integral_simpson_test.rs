use assert_approx_eq::assert_approx_eq;
use sequential_integration::calculate_single_integral_simpson;

#[test]
fn calculate_single_integral_simpson_test() {
    let equation = "max(sqrt(1 - x^2))";
    let first_integral_begin = -1.;
    let first_integral_end = 1.;
    let first_integral_step = 0.05;

    let result = calculate_single_integral_simpson(
        equation,
        first_integral_begin,
        first_integral_end,
        first_integral_step,
    )
    .unwrap();

    println!(
        "result: {}, expected: {}, diff: {}",
        result,
        std::f64::consts::FRAC_PI_2,
        result - std::f64::consts::FRAC_PI_2
    );

    assert_approx_eq!(result, std::f64::consts::FRAC_PI_2, 1e-2);
}
