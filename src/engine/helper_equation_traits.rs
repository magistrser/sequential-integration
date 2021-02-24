use super::{Bounds, CalculationResult, CalculationStep};
use crate::errors::Result;

pub trait EquationOfOneVariable {
    fn calculate(&self, value: CalculationStep, bounds: Bounds) -> Result<CalculationResult>;
}

pub trait EquationOfTwoVariable {
    fn calculate(
        &self,
        value1: CalculationStep,
        bounds1: Bounds,
        value2: CalculationStep,
        bounds2: Bounds,
    ) -> Result<CalculationResult>;
}

pub trait EquationOfThreeVariable {
    fn calculate(
        &self,
        value1: CalculationStep,
        bounds2: Bounds,
        value2: CalculationStep,
        bounds2: Bounds,
        value3: CalculationStep,
        bounds3: Bounds,
    ) -> Result<CalculationResult>;
}
