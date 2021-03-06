use assert_approx_eq::assert_approx_eq;
use sequential_integration::calculate_double_integral_simpson;

#[test]
fn calculate_double_integral_simpson_circel() {
    let equation = |_x, _y| 1.;
    let first_integral_begin = -1.;
    let first_integral_end = 1.;
    let first_integral_step = 0.005;

    let second_integral_begin = |_x| -0.;
    let second_integral_end = |x: f64| (1. - x.powf(2.)).sqrt();
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

    println!(
        "result: {}, expected: {}, diff: {}",
        result,
        std::f64::consts::FRAC_PI_2,
        result - std::f64::consts::FRAC_PI_2
    );

    assert_approx_eq!(result, std::f64::consts::FRAC_PI_2, 1e-2);
}

#[test]
fn calculate_double_integral_simpson_not_const_equation() {
    let equation = |x: f64, y: f64| x.powf(2.) / y.powf(2.);
    let first_integral_begin = 1.;
    let first_integral_end = 2.;
    let first_integral_step = 0.001;

    let second_integral_begin = |x: f64| 1. / x;
    let second_integral_end = |x: f64| x;
    let second_integral_step = 0.001;

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

    println!(
        "result: {}, expected: {}, diff: {}",
        result,
        2.25,
        result - 2.25
    );

    assert_approx_eq!(result, 2.25, 1e-2);
}
