mod step_type;
pub use step_type::StepType;

use crate::errors::Result;

pub trait RangeGenerator {
    fn next(&mut self) -> Result<step_type::StepType>;
}
