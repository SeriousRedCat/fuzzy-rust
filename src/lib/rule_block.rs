use crate::norm::NormTrait;
use crate::rule::RuleTrait;

pub trait RuleBlockTrait<Rule> {
    fn add_rule(&mut self, rule: Rule);
    fn compute(&mut self);
}

#[derive(Default)]
pub struct RuleBlock<Rule, Conjunction, Disjunction, Implication>
where
    Conjunction: NormTrait,
    Disjunction: NormTrait,
    Implication: NormTrait,
{
    rules: std::vec::Vec<Rule>,

    conjunction: Conjunction,
    disjunction: Disjunction,
    implication: Implication,
}

impl<Rule, Conjunction: NormTrait, Disjunction: NormTrait, Implication: NormTrait>
    RuleBlock<Rule, Conjunction, Disjunction, Implication>
{
    pub fn new(
        conjunction: Conjunction,
        disjunction: Disjunction,
        implication: Implication,
    ) -> Self {
        RuleBlock {
            rules: Default::default(),
            conjunction,
            disjunction,
            implication,
        }
    }
}

impl<Rule: RuleTrait, Conjunction: NormTrait, Disjunction: NormTrait, Implication: NormTrait>
    RuleBlockTrait<Rule> for RuleBlock<Rule, Conjunction, Disjunction, Implication>
{
    fn add_rule(&mut self, rule: Rule) {}
    fn compute(&mut self) {
        for rule in &self.rules {
            rule.activate_with(&self.conjunction, &self.disjunction);
        }
    }
}
