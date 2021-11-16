use std::marker::PhantomData;

use crate::rule_block::RuleBlock;
use crate::rule_block::RuleBlockTrait;
use crate::term::TermTrait;
use crate::variable::VariableTrait;
use crate::SharedPtr;
use std::rc::Rc;

pub struct Engine {
    inputs: std::collections::HashMap<String, SharedPtr<dyn VariableTrait>>,
    outputs: std::collections::HashMap<String, SharedPtr<dyn VariableTrait>>,

    rule_block: RuleBlock,
    // _marker: PhantomData<T>,
    // _marker2: PhantomData<Rule>,
}

impl Engine {
    pub fn new() -> Self {
        Engine {
            inputs: Default::default(),
            outputs: Default::default(),
            rule_block: Default::default(),
            // _marker: Default::default(),
            // _marker2: Default::default(),
        }
    }

    pub fn set_input_value(_id: &String, value: f64) {}

    pub fn register_input(&mut self, id: String, variable: SharedPtr<dyn VariableTrait>) {
        self.inputs.insert(id.clone(), variable);
        // self.inputs.get_mut(&id).unwrap()
    }

    pub fn register_output(&mut self, id: String, variable: SharedPtr<dyn VariableTrait>) {
        self.outputs.insert(id.clone(), variable);
        // self.outputs.get_mut(&id).unwrap()
    }

    pub fn register_rule_block(&mut self, block: RuleBlock) {
        self.rule_block = block;
    }

    pub fn process(&mut self) {
        for _input in self.inputs.values() {
            // let locked = input.
            // input.fuzzify(input.value());
        }

        self.rule_block.compute();
    }

    pub fn input(&self, id: &String) -> Option<&SharedPtr<dyn VariableTrait>> {
        self.inputs.get(id)
    }

    pub fn output(&self, id: &String) -> Option<&SharedPtr<dyn VariableTrait>> {
        self.outputs.get(id)
    }
}
