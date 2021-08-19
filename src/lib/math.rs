pub fn bell(x: f64, center: f64, width: f64, slope: f64) -> f64 {
    1.0 / (1.0 + ((x - center) / width).abs().powf(2.0 * slope))
}
