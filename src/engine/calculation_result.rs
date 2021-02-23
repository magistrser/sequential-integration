#[derive(Debug, Copy, Clone)]
pub struct CalculationResult {
    pub common: f64,
    pub last: f64,
}

impl std::ops::AddAssign for CalculationResult {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            common: self.common + other.common,
            last: self.last + other.last,
        };
    }
}

impl CalculationResult {
    pub fn new() -> Self {
        Self {
            common: 0.,
            last: 0.,
        }
    }

    pub fn add_common(&mut self, value: f64) {
        self.common += value;
    }

    pub fn add_last(&mut self, value: f64) {
        self.last += value;
    }
}
