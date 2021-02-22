use fehler::{throw, throws};
use mexprp::{Answer, Context, Expression};

use crate::errors::{self, Error};

#[throws]
pub fn calculate_expression_one_value_result(
    context: &Context<f64>,
    expression: &Expression<f64>,
) -> f64 {
    match expression.eval_ctx(context)? {
        Answer::Single(value) => value,
        _ => throw!(errors::EquationWithMultipleResult {
            equation: expression.string.clone()
        }
        .build()),
    }
}
