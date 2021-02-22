#[derive(Debug)]
pub enum StepType {
    Common(f64),
    Last(f64),
    NoStep,
}
