use snafu::{Backtrace, Snafu};

#[derive(Debug, Snafu)]
#[snafu(visibility(pub(crate)))]
pub enum Error {
    #[snafu(display("RangeGenerator step{} out of end bound{}", step, end))]
    RangeGeneratorOutOfBounds {
        step: f64,
        end: f64,
        backtrace: Backtrace,
    },

    #[snafu(display("Begin bound{} greater than end bound{}", begin, end))]
    BeginBoundGreaterThanEndBound {
        begin: f64,
        end: f64,
        backtrace: Backtrace,
    },
}

pub type Result<T, E = Error> = std::result::Result<T, E>;
