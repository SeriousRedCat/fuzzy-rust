use crate::math;
use crate::variable::VariableTrait;
use crate::SharedPtr;
use crate::WeakPtr;
use std::cell::RefCell;
use std::rc::Rc;
use std::rc::Weak;

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
    variable: &SharedPtr<dyn VariableTrait>,
    name: String,
    range: Range,
    center: f64,
    width: f64,
    slope: f64,
) -> Term<Range> {
    let term = Term {
        range,
        name,
        term_type: TermType::Bell(center, width, slope),
        variable: Rc::downgrade(variable),
    };
    // variable.borrow_mut().add_term(term);
    term
}

// pub fn binary<'a, Range: std::ops::RangeBounds<f64>>(
//     // variable: &'a impl VariableTrait,
//     name: String,
//     range: Range,
//     start: std::ops::Bound<f64>,
//     positive: bool,
// ) -> Term<'a, Range> {
//     Term {
//         range,
//         name,
//         term_type: TermType::Binary(start, positive),
//         variable: None,
//     }
// }

pub trait TermTrait: std::fmt::Display {
    fn name(&self) -> &String;
    fn membership(&self, x: f64) -> f64;
    fn min(&self) -> std::ops::Bound<&f64>;
    fn max(&self) -> std::ops::Bound<&f64>;

    // fn set_parent(&mut self, parent: &Self::Variable);

    fn variable(&self) -> Option<SharedPtr<dyn VariableTrait>>;
}

pub struct Term<Range: std::ops::RangeBounds<f64>> {
    range: Range,
    name: String,
    term_type: TermType,
    variable: WeakPtr<dyn VariableTrait>,
}

impl<Range: std::ops::RangeBounds<f64>> TermTrait for Term<Range> {
    // type Variable = Variable;
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

    // fn set_parent(&mut self, parent: &Self::Variable) {
    //     self.variable = Some(parent);
    // }

    fn variable(&self) -> Option<SharedPtr<dyn VariableTrait>> {
        self.variable.upgrade()
    }
}

impl<Range: std::ops::RangeBounds<f64>>  std::fmt::Display for Term<Range> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
write!(f, "{}", self.name)
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
