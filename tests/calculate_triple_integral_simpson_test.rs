use assert_approx_eq::assert_approx_eq;
use sequential_integration::calculate_triple_integral_simpson;

#[test]
fn calculate_triple_integral_simpson_sphere() {
    let equation = "1";
    let first_integral_begin = -1.;
    let first_integral_end = 1.;
    let first_integral_step = 0.01;

    let second_integral_begin = "0";
    let second_integral_end = "max(sqrt(1 - x^2))";
    let second_integral_step = 0.01;

    let third_integral_begin = "0";
    let third_integral_end = "max(sqrt(1 - x^2 - y^2))";
    let third_integral_step = 0.01;

    let result = calculate_triple_integral_simpson(
        equation,
        first_integral_begin,
        first_integral_end,
        first_integral_step,
        second_integral_begin,
        second_integral_end,
        second_integral_step,
        third_integral_begin,
        third_integral_end,
        third_integral_step,
    )
    .unwrap();

    println!(
        "result: {}, expected: {}, diff: {}",
        result,
        std::f64::consts::FRAC_PI_3,
        result - std::f64::consts::FRAC_PI_3
    );

    assert_approx_eq!(result, std::f64::consts::FRAC_PI_3, 2e-2);
}

#[test]
fn calculate_triple_integral_simpson_cube() {
    let equation = "1";
    let first_integral_begin = -1.;
    let first_integral_end = 1.;
    let first_integral_step = 0.05;

    let second_integral_begin = "0";
    let second_integral_end = "2";
    let second_integral_step = 0.05;

    let third_integral_begin = "0";
    let third_integral_end = "2";
    let third_integral_step = 0.05;

    let result = calculate_triple_integral_simpson(
        equation,
        first_integral_begin,
        first_integral_end,
        first_integral_step,
        second_integral_begin,
        second_integral_end,
        second_integral_step,
        third_integral_begin,
        third_integral_end,
        third_integral_step,
    )
    .unwrap();

    println!(
        "result: {}, expected: {}, diff: {}",
        result,
        8.,
        result - 8.
    );

    assert_approx_eq!(result, 8., 1e-2);
}

#[test]
fn calculate_triple_integral_simpson_not_const_equation_positive() {
    let equation = "x ^ 2 + y ^ 2 + z ^ 2";
    let first_integral_begin = -1.;
    let first_integral_end = 1.;
    let first_integral_step = 0.005;

    let second_integral_begin = "0";
    let second_integral_end = "x / 2";
    let second_integral_step = 0.005;

    let third_integral_begin = "0";
    let third_integral_end = "x^2 + y";
    let third_integral_step = 0.005;

    let result = calculate_triple_integral_simpson(
        equation,
        first_integral_begin,
        first_integral_end,
        first_integral_step,
        second_integral_begin,
        second_integral_end,
        second_integral_step,
        third_integral_begin,
        third_integral_end,
        third_integral_step,
    )
    .unwrap();

    let expected_result = 79. / 840.;
    println!(
        "result: {}, expected: {}, diff: {}",
        result,
        expected_result,
        result - expected_result
    );

    assert_approx_eq!(result, expected_result, 1e-2);
}

#[test]
fn calculate_triple_integral_simpson_not_const_equation_negative() {
    let equation = "x ^ 2 + y ^ 2 + z ^ 2";
    let first_integral_begin = -1.;
    let first_integral_end = 1.;
    let first_integral_step = 0.005;

    let second_integral_begin = "x / 2";
    let second_integral_end = "0";
    let second_integral_step = 0.005;

    let third_integral_begin = "0";
    let third_integral_end = "x^2 + y";
    let third_integral_step = 0.005;

    let result = calculate_triple_integral_simpson(
        equation,
        first_integral_begin,
        first_integral_end,
        first_integral_step,
        second_integral_begin,
        second_integral_end,
        second_integral_step,
        third_integral_begin,
        third_integral_end,
        third_integral_step,
    )
    .unwrap();

    let expected_result = -79. / 840.;
    println!(
        "result: {}, expected: {}, diff: {}",
        result,
        expected_result,
        result - expected_result
    );

    assert_approx_eq!(result, expected_result, 1e-2);
}

#[test]
fn calculate_triple_integral_simpson_not_const_from_larger_to_smile() {
    let equation = "x ^ 2 + y ^ 2 + z ^ 2";
    let first_integral_begin = 1.;
    let first_integral_end = -1.;
    let first_integral_step = 0.01;

    let second_integral_begin = "x";
    let second_integral_end = "x / 2";
    let second_integral_step = 0.01;

    let third_integral_begin = "x^2 + y";
    let third_integral_end = "0";
    let third_integral_step = 0.01;

    let result = calculate_triple_integral_simpson(
        equation,
        first_integral_begin,
        first_integral_end,
        first_integral_step,
        second_integral_begin,
        second_integral_end,
        second_integral_step,
        third_integral_begin,
        third_integral_end,
        third_integral_step,
    )
    .unwrap();

    let expected_result = -107. / 280.;
    println!(
        "result: {}, expected: {}, diff: {}",
        result,
        expected_result,
        result - expected_result
    );

    assert_approx_eq!(result, expected_result, 3e-2);
}
