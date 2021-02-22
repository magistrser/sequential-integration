pub mod simpson;

use super::{
    helper_equation_traits::{EquationOfThreeVariable, EquationOfTwoVariable},
    range_generator::RangeGenerator,
};
use crate::errors::Result;

pub trait GetQuadratureRange {
    fn get_range_generator(a: f64, b: f64, h: f64) -> Result<Box<dyn RangeGenerator>>;
}

pub trait FinalizeCalculation {
    fn finalize(&self, result: f64) -> Result<f64>;
}

pub trait GetStepSizeDoubleIntegral {
    fn get_step_size(&self) -> (f64, f64);
}

pub trait QuadratureDoubleIntegral
where
    Self: GetQuadratureRange
        + FinalizeCalculation
        + GetStepSizeDoubleIntegral
        + EquationOfTwoVariable
        + Clone,
{
}

pub trait GetStepSizeTripleIntegral {
    fn get_step_size(&self) -> (f64, f64, f64);
}

pub trait QuadratureTripleIntegral
where
    Self: GetQuadratureRange
        + FinalizeCalculation
        + GetStepSizeTripleIntegral
        + EquationOfThreeVariable
        + Clone,
{
}
