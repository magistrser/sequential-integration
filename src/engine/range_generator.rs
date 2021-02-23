pub use super::CalculationStep;

use crate::errors::Result;

pub trait RangeGenerator {
    fn next(&mut self) -> Result<CalculationStep>;
}
