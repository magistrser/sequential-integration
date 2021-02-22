use snafu::{Backtrace, GenerateBacktrace, Snafu};

#[derive(Debug, Snafu)]
#[snafu(visibility(pub(crate)))]
pub enum Error {
    #[snafu(display("mexprp error: {}", description))]
    MexprpError {
        description: String,
        backtrace: Backtrace,
    },

    #[snafu(display(
        "Equation with multiple result not supported for integral bounds, equation: {}",
        equation
    ))]
    EquationWithMultipleResult {
        equation: String,
        backtrace: Backtrace,
    },

    #[snafu(display("RangeGenerator step{} out of b bound{}", step, b))]
    RangeGeneratorOutOfBounds {
        step: f64,
        b: f64,
        backtrace: Backtrace,
    },
}

pub type Result<T, E = Error> = std::result::Result<T, E>;

impl From<mexprp::ParseError> for Error {
    fn from(error: mexprp::ParseError) -> Self {
        Error::MexprpError {
            description: error.to_string(),
            backtrace: Backtrace::generate(),
        }
    }
}

impl From<mexprp::MathError> for Error {
    fn from(error: mexprp::MathError) -> Self {
        Error::MexprpError {
            description: error.to_string(),
            backtrace: Backtrace::generate(),
        }
    }
}

impl From<mexprp::EvalError> for Error {
    fn from(error: mexprp::EvalError) -> Self {
        Error::MexprpError {
            description: error.to_string(),
            backtrace: Backtrace::generate(),
        }
    }
}
