use crate::norm::Norm;
use crate::rule::RuleTrait;

pub trait RuleBlockTrait {
    fn add_rule(&mut self, rule: Box<dyn RuleTrait>);
    fn compute(&mut self);
}

#[derive(Default)]
pub struct RuleBlock {
    rules: std::vec::Vec<Box<dyn RuleTrait>>,

    conjunction: Norm,
    disjunction: Norm,
    implication: Norm,
}

impl RuleBlock {
    pub fn new(conjunction: Norm, disjunction: Norm, implication: Norm) -> Self {
        RuleBlock {
            rules: Default::default(),
            conjunction,
            disjunction,
            implication,
        }
    }
}

impl RuleBlockTrait for RuleBlock {
    fn add_rule(&mut self, _rule: Box<dyn RuleTrait>) {}
    fn compute(&mut self) {
        for rule in &self.rules {
            rule.activate_with(&self.conjunction, &self.disjunction);
        }
    }
}
