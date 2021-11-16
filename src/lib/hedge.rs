pub trait HedgeTrait {
    fn hedge(x: f64) -> f64;
}

pub struct Hedge {}

impl HedgeTrait for Hedge {
    fn hedge(_x: f64) -> f64 {
        0f64
    }
}
