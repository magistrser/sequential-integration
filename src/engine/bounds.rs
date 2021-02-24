use fehler::throws;
use snafu::ensure;

use crate::errors::{self, Error};

#[derive(Copy, Clone)]
pub struct Bounds {
    pub begin: f64,
    pub end: f64,
}

impl Bounds {
    #[throws]
    pub fn new(begin: f64, end: f64) -> Self {
        ensure!(
            begin <= end,
            errors::BeginBoundGreaterThanEndBound { begin, end }
        );

        Self { begin, end }
    }
}
