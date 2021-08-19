use crate::math;

enum TermType {
    Bell(f64, f64, f64),
    Binary,
    // Concave,
    // Constant,
    // Cosine,
    // Discrete,
    // Gaussian,
    // GaussianProduct,
    // Linear,
    // PiShape,
    // Ramp,
    // Rectangle,
    // Sigmoid,
    // SigmoidDifference,
    // SigmoidProduct,
    // Spike,
    // SShape,
    // Trapezoid,
    // Triangle,
    // ZShape
}

impl TermType {
    pub fn value(&self, x: f64) -> f64 {
        match &self {
            TermType::Bell(center, width, slope) => math::bell(x, *center, *width, *slope),
            TermType::Binary => 3.0,
        }
    }
}

pub fn bell<Range: std::ops::RangeBounds<f64>>(range: Range, center: f64, width: f64, slope: f64) -> impl Term {
    TermImpl {
        range,
        name: "bell".to_string(),
        term_type: TermType::Bell(center, width, slope)
    }
}

pub trait Term {
    fn value(&self, x: f64) -> f64;
    fn min(&self) -> std::ops::Bound<&f64>;
    fn max(&self) -> std::ops::Bound<&f64>;
}

pub struct TermImpl<Range: std::ops::RangeBounds<f64>> {
    range: Range,
    name: String,
    term_type: TermType,
}

impl<Range: std::ops::RangeBounds<f64>> Term for TermImpl<Range> {
    fn value(&self, x: f64) -> f64 {
        if self.range.contains(&x) {
            self.term_type.value(x)
        } else {
            0.0
        }
    }

    fn min(&self) -> std::ops::Bound<&f64> {
        self.range.start_bound()
    }

    fn max(&self) -> std::ops::Bound<&f64> {
        self.range.end_bound()
    }
}

// KCOV_SKIP_START
#[cfg(test)]
mod tests {
    use super::*;
    use std::ops::RangeBounds;

    #[test]
    fn test_constructors() {
        let constructed = bell(.., 2.0, 5.0, 10.0);
        let expected = TermImpl{
            range: ..,
            name: "bell".to_string(),
            term_type: TermType::Bell(2.0, 5.0, 10.0)
        };

        assert_eq!(constructed.min(), expected.range.start_bound());
        assert_eq!(constructed.max(), expected.range.end_bound());
    }
}
// KCOV_SKIP_END
