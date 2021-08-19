use crate::Expression;
use crate::Norm;

trait Antecedent {
    fn activation_degree(conjunction: impl Norm, disjunction: impl Norm, expression: impl Expression) -> f64;
}