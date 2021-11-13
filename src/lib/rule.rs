use crate::norm::NormTrait;

pub trait RuleTrait {
    fn activate_with(&self, conjunction: &impl NormTrait, disjunction: &impl NormTrait);
    fn trigger(implication: impl NormTrait);
    fn activation_degree() -> f64;

    fn to_string() -> String;
}

#[derive(Default)]
pub struct Rule<Antecedent, Consequent> {
    antecedent: Antecedent,
    consequent: Consequent,
    weigth: f64,
}

impl<Antecedent, Consequent> Rule<Antecedent, Consequent> {
    pub fn new(antecedent: Antecedent, consequent: Consequent) -> Self {
        Rule {
            antecedent,
            consequent,
            weigth: 1f64,
        }
    }
}

impl<Antecedent, Consequent> RuleTrait for Rule<Antecedent, Consequent> {
    fn activate_with(&self, conjunction: &impl NormTrait, disjunction: &impl NormTrait) {}
    fn trigger(implication: impl NormTrait) {}
    fn activation_degree() -> f64 {
        0.0
    }

    fn to_string() -> String {
        format!("if then")
    }
}
