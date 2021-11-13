use std::marker::PhantomData;

use crate::rule_block::RuleBlockTrait;
use crate::term::TermTrait;
use crate::variable::VariableTrait;

pub struct Engine<Id, Input, Output, T, RuleBlock, Rule> {
    inputs: std::collections::HashMap<Id, Input>,
    outputs: std::collections::HashMap<Id, Output>,

    rule_block: RuleBlock,

    _marker: PhantomData<T>,
    _marker2: PhantomData<Rule>,
}

impl<Id, Input, Output, T, RuleBlock, Rule> Engine<Id, Input, Output, T, RuleBlock, Rule>
where
    Id: Eq + std::hash::Hash,
    Input: VariableTrait<T>,
    Output: VariableTrait<T>,
    T: TermTrait,
    RuleBlock: RuleBlockTrait<Rule> + std::default::Default,
{
    pub fn new() -> Self {
        Engine {
            inputs: Default::default(),
            outputs: Default::default(),
            rule_block: Default::default(),
            _marker: Default::default(),
            _marker2: Default::default(),
        }
    }

    pub fn set_input_value(id: &Id, value: f64) {}

    pub fn register_input(&mut self, id: Id, variable: Input) {
        self.inputs.insert(id, variable);
    }

    pub fn register_output(&mut self, id: Id, variable: Output) {
        self.outputs.insert(id, variable);
    }

    pub fn register_rule_block(&mut self, block: RuleBlock) {
        self.rule_block = block;
    }

    pub fn process(&mut self) {
        for input in self.inputs.values() {
            input.fuzzify(input.value());
        }

        self.rule_block.compute();
    }

    pub fn input(&self, id: Id) -> Option<&Input> {
        self.inputs.get(&id)
    }
}
