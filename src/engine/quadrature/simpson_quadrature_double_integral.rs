use fehler::throws;
use mexprp::{Context, Expression};

use super::{
    FinalizeCalculation, GetQuadratureRange, GetStepSizeDoubleIntegral, QuadratureDoubleIntegral,
};
use crate::{
    engine::{
        helper_equation_traits::{Bounds, EquationOfTwoVariable},
        utils,
    },
    errors::Error,
};

pub struct SimpsonQuadratureDoubleIntegral {
    equation: Expression<f64>,
    h: f64,
    k: f64,
}

impl Clone for SimpsonQuadratureDoubleIntegral {
    fn clone(&self) -> Self {
        Self::new(self.equation.string.as_str(), self.h, self.k).unwrap()
    }
}

impl SimpsonQuadratureDoubleIntegral {
    #[throws]
    pub fn new(equation: &str, h: f64, k: f64) -> Self {
        let equation = Expression::parse(equation)?;
        Self { equation, h, k }
    }

    #[throws]
    fn calculate_simpson(&self, x_values: [f64; 3], y_values: [f64; 3]) -> f64 {
        let mut context = Context::new();
        let mut f = vec![];

        for x in x_values.iter() {
            let mut f_y = vec![];
            for y in y_values.iter() {
                context.set_var("x", *x);
                context.set_var("y", *y);

                f_y.push(utils::calculate_expression_one_value_result(
                    &context,
                    &self.equation,
                )?);
            }
            f.push(f_y);
        }

        let result = f[0][0]
            + f[2][0]
            + f[0][2]
            + f[2][2]
            + 4. * (f[1][0] + f[0][1] + f[2][1] + f[1][2])
            + 16. * f[1][1];
        result
    }
}

impl EquationOfTwoVariable for SimpsonQuadratureDoubleIntegral {
    #[throws]
    fn calculate(&self, x: f64, _: Bounds, y: f64, _: Bounds) -> f64 {
        let x0 = x - self.h;
        let x1 = x;
        let x2 = x + self.h;

        let y0 = y - self.k;
        let y1 = y;
        let y2 = y + self.k;

        let x_values = [x0, x1, x2];
        let y_values = [y0, y1, y2];

        self.calculate_simpson(x_values, y_values)?
    }

    #[throws]
    fn calculate_last(&self, x: f64, bounds_x: Bounds, y: f64, bounds_y: Bounds) -> f64 {
        let x0 = x - self.h;
        let x2 = bounds_x.1;
        let x1 = (x2 - x0) / 2.;

        let y0 = y - self.k;
        let y2 = bounds_y.1;
        let y1 = (y2 - y0) / 2.;

        let x_values = [x0, x1, x2];
        let y_values = [y0, y1, y2];

        self.calculate_simpson(x_values, y_values)?
    }
}

impl FinalizeCalculation for SimpsonQuadratureDoubleIntegral {
    #[throws]
    fn finalize(&self, result: f64) -> f64 {
        self.h * self.k * result / 9.
    }
}

impl GetStepSizeDoubleIntegral for SimpsonQuadratureDoubleIntegral {
    fn get_step_size(&self) -> (f64, f64) {
        (self.h, self.k)
    }
}

impl GetQuadratureRange for SimpsonQuadratureDoubleIntegral {
    #[throws]
    fn get_range(a: f64, b: f64, h: f64) -> Vec<f64> {
        let mut range = vec![];
        range.reserve(((b - a) / h) as usize + 1);

        let mut x = a + h;
        while x < b {
            range.push(x);
            x += 2. * h;
        }

        range
    }
}

impl QuadratureDoubleIntegral for SimpsonQuadratureDoubleIntegral {}
