pub trait NormTrait {
    fn compute(left: f64, right: f64) -> f64;
}

#[derive(Default)]
pub struct Norm {}

impl NormTrait for Norm {
    fn compute(left: f64, right: f64) -> f64 {
        1f64
    }
}
