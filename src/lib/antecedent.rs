use std::fmt::Display;
use std::path::Ancestors;

use crate::expression::ExpressionTrait;
use crate::expression::Proposition;
use crate::norm::NormTrait;

pub trait AntecedentTrait: std::fmt::Display {
    // fn set_expression(&mut self, expression: Expression);
    fn activation_degree(
        conjunction: impl NormTrait,
        disjunction: impl NormTrait,
        expression: impl ExpressionTrait,
    ) -> f64;
}

#[derive(Default)]
pub struct Antecedent<T: ExpressionTrait> {
    expression: T,
}

impl<T: ExpressionTrait> Antecedent<T> {
    pub fn new(expression: T) -> Self {
        Antecedent {
            expression
        }
    }
}

impl<T: ExpressionTrait> AntecedentTrait for Antecedent<T> {
    // fn set_expression(&mut self, expression: Expression) {
    //     self.expression = expression;
    // }
    fn activation_degree(
        _conjunction: impl NormTrait,
        _disjunction: impl NormTrait,
        _expression: impl ExpressionTrait,
    ) -> f64 {
        0.0
    }
}

impl<T: ExpressionTrait> std::fmt::Display for Antecedent<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.expression)
    }
}
