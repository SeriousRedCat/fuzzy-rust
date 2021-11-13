use crate::expression::ExpressionTrait;
use crate::norm::NormTrait;

pub trait AntecedentTrait {
    // fn set_expression(&mut self, expression: Expression);
    fn activation_degree(
        conjunction: impl NormTrait,
        disjunction: impl NormTrait,
        expression: impl ExpressionTrait,
    ) -> f64;
}

#[derive(Default)]
pub struct Antecedent<Expression> {
    expression: Expression,
}

impl<Expression> Antecedent<Expression> {
    pub fn new(expression: Expression) -> Self {
        Antecedent { expression }
    }
}

impl<Expression> AntecedentTrait for Antecedent<Expression> {
    // fn set_expression(&mut self, expression: Expression) {
    //     self.expression = expression;
    // }
    fn activation_degree(
        conjunction: impl NormTrait,
        disjunction: impl NormTrait,
        expression: impl ExpressionTrait,
    ) -> f64 {
        0.0
    }
}
