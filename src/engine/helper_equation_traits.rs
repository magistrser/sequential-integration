use crate::errors::Result;

pub type Bounds = (f64, f64);

pub trait EquationOfOneVariable {
    fn calculate(&self, value: f64, bounds: Bounds) -> Result<f64>;
    fn calculate_last(&self, _: f64, _: Bounds) -> Result<f64>;
}

pub trait EquationOfTwoVariable {
    fn calculate(&self, value1: f64, bounds1: Bounds, value2: f64, bounds2: Bounds) -> Result<f64>;
    fn calculate_last(&self, _: f64, _: Bounds, _: f64, _: Bounds) -> Result<f64>;
}

pub trait EquationOfThreeVariable {
    fn calculate(
        &self,
        value1: f64,
        bounds2: Bounds,
        value2: f64,
        bounds2: Bounds,
        value3: f64,
        bounds3: Bounds,
    ) -> Result<f64>;

    fn calculate_last(
        &self,
        _: f64,
        _: Bounds,
        _: f64,
        _: Bounds,
        _: f64,
        _: Bounds,
    ) -> Result<f64>;
}
