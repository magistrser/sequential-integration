use crate::engine::{Bounds, CalculationStep};

pub struct SimpsonPoints {
    pub v0: f64,
    pub v1: f64,
    pub v2: f64,
    pub h: f64,
}

impl SimpsonPoints {
    pub fn generate(
        step: CalculationStep,
        bounds: Bounds,
        step_size: f64,
        is_last_step: &mut bool,
    ) -> Self {
        let v0 = *step;
        let (v1, v2, h) = if step.is_last() {
            *is_last_step = true;

            let v2 = bounds.end;
            let h = (v2 - v0) / 2.;
            let v1 = v0 + h;
            (v1, v2, h)
        } else {
            let v1 = v0 + step_size;
            let v2 = v1 + step_size;
            (v1, v2, step_size)
        };

        Self { v0, v1, v2, h }
    }
}
