pub trait NormTrait {
    fn compute(&self, left: f64, right: f64) -> f64;
}

#[derive(Default)]
pub struct Norm {}

impl NormTrait for Norm {
    fn compute(&self, _left: f64, _right: f64) -> f64 {
        1f64
    }
}
