use assert_approx_eq::assert_approx_eq;
use sequential_integration::calculate_double_integral_simpson;

#[test]
fn calculate_double_integral_simpson_test() {
    let equation = "1";
    let first_integral_begin = -1.;
    let first_integral_end = 1.;
    let first_integral_step = 0.005;

    let second_integral_begin = "0";
    let second_integral_end = "max(sqrt(1 - x^2))";
    let second_integral_step = 0.005;

    let result = calculate_double_integral_simpson(
        equation,
        first_integral_begin,
        first_integral_end,
        first_integral_step,
        second_integral_begin,
        second_integral_end,
        second_integral_step,
    )
    .unwrap();

    assert_approx_eq!(result, std::f64::consts::FRAC_PI_2, 1e-2);
}
