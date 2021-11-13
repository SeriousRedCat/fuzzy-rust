use crate::math;
// use crate::variable::VariableTrait;

enum TermType {
    Bell(f64, f64, f64),
    Binary(std::ops::Bound<f64>, bool),
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
    pub fn membership(&self, x: f64) -> f64 {
        match &self {
            TermType::Bell(center, width, slope) => math::bell(x, *center, *width, *slope),
            TermType::Binary(start, positive) => math::binary(x, *start, *positive),
        }
    }
}

pub fn bell<Range: std::ops::RangeBounds<f64>>(
    name: String,
    range: Range,
    center: f64,
    width: f64,
    slope: f64,
) -> impl TermTrait {
    Term {
        range,
        name,
        term_type: TermType::Bell(center, width, slope),
    }
}

pub fn binary<Range: std::ops::RangeBounds<f64>>(
    name: String,
    range: Range,
    start: std::ops::Bound<f64>,
    positive: bool,
) -> impl TermTrait {
    Term {
        range,
        name,
        term_type: TermType::Binary(start, positive),
    }
}

pub trait TermTrait {
    fn name(&self) -> &String;
    fn membership(&self, x: f64) -> f64;
    fn min(&self) -> std::ops::Bound<&f64>;
    fn max(&self) -> std::ops::Bound<&f64>;

    // fn variable() -> Option<&VariableTrait<Term>>;
}

pub struct Term<Range: std::ops::RangeBounds<f64>> {
    range: Range,
    name: String,
    term_type: TermType,
    // variable: Option<&VariableTrait<Term>>,
}

impl<Range: std::ops::RangeBounds<f64>> TermTrait for Term<Range> {
    fn name(&self) -> &String {
        &self.name
    }

    fn membership(&self, x: f64) -> f64 {
        if self.range.contains(&x) {
            self.term_type.membership(x)
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
        let constructed = bell("bell".to_string(), .., 2.0, 5.0, 10.0);
        let expected = Term {
            range: ..,
            name: "bell".to_string(),
            term_type: TermType::Bell(2.0, 5.0, 10.0),
        };

        assert_eq!(constructed.min(), expected.range.start_bound());
        assert_eq!(constructed.max(), expected.range.end_bound());
    }
}
// KCOV_SKIP_END
