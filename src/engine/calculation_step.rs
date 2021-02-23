#[derive(Debug, Copy, Clone)]
pub enum CalculationStep {
    Common(f64),
    Last(f64),
}

impl std::ops::Deref for CalculationStep {
    type Target = f64;

    fn deref(&self) -> &Self::Target {
        match self {
            CalculationStep::Common(value) => value,
            CalculationStep::Last(value) => value,
        }
    }
}

impl CalculationStep {
    pub fn is_last(&self) -> bool {
        match self {
            CalculationStep::Last(_) => true,
            _ => false,
        }
    }
}
