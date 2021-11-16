use std::ops::RangeFull;

use crate::fuzzy::make_shared;
use fuzzy;
use fuzzy::antecedent::Antecedent;
use fuzzy::consequent::Consequent;
use fuzzy::expression::Proposition;
use fuzzy::hedge::Hedge;
use fuzzy::norm::Norm;
use fuzzy::rule::{Rule, RuleTrait};
use fuzzy::rule_block::{RuleBlock, RuleBlockTrait};
use fuzzy::term::{Term, TermTrait};
use fuzzy::variable::{Variable, VariableTrait};
use fuzzy::SharedPtr;
use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    let mut engine = fuzzy::engine::Engine::new();
    // let mut engine: fuzzy::engine::Engine<String, Variable, Variable,
    // RuleBlock<Rule<Antecedent<Proposition<Hedge>>, Consequent>, Norm, Norm, Norm>, Rule<Antecedent<Proposition<Hedge>> = fuzzy::engine::Engine::new();
    // let service = engine.register_input("service", fuzzy::variable::Variable::new("service".to_string()));

    // let mut service = ;
    engine.register_input(
        "service".to_string(),
        make_shared!(fuzzy::variable::Variable::new("service".to_string())),
    );
    let service = engine.input(&"service".to_string()).unwrap();
    // let mut food = engine.register_input("food", fuzzy::variable::Variable::new("food".to_string()));
    // let mut tip = engine.register_output("tip", fuzzy::variable::Variable::new("tip".to_string()));

    // // engine.register_input("service", service);
    // // engine.register_input("food", food);
    // // engine.register_output("tip", tip);

    let poor: SharedPtr<dyn TermTrait> = make_shared!(fuzzy::term::bell(
        service,
        "poor".to_string(),
        ..,
        5.0,
        2.0,
        3.0
    ));
    let good: SharedPtr<dyn TermTrait> = make_shared!(fuzzy::term::bell(
        service,
        "good".to_string(),
        0f64..10f64,
        5.0,
        2.0,
        3.0
    ));
    let excelent: SharedPtr<dyn TermTrait> = make_shared!(fuzzy::term::bell(
        service,
        "excelent".to_string(),
        ..,
        5.0,
        2.0,
        3.0
    ));

    {
        let mut s = service.borrow_mut();
        s.add_term(poor.clone());

        // service.add_term(poor);
        s.add_term(good);
        s.add_term(excelent);
        println!(
            "Poor {}",
            s.term(&"poor".to_string())
                .unwrap()
                .borrow()
                .membership(6f64)
        );
    }

    // //     for i in 0..10 {
    // //     println!("Poor {}", poor.membership(i.into()));
    // // }

    // let rancid = fuzzy::term::bell(engine
    //     .input("food")
    //     .unwrap(), "rancid".to_string(), .., 5.0, 2.0, 3.0);
    // let delicious = fuzzy::term::bell(engine
    //     .input("food")
    //     .unwrap(), "delicious".to_string(), .., 5.0, 2.0, 3.0);

    // // food.add_term(rancid);
    // // food.add_term(delicious);

    // let cheap = fuzzy::term::bell(engine
    //     .output("tip")
    //     .unwrap(), "cheap".to_string(), .., 5.0, 2.0, 3.0);
    // let average = fuzzy::term::bell(engine
    //     .output("tip")
    //     .unwrap(), "average".to_string(), .., 5.0, 2.0, 3.0);
    // let generous = fuzzy::term::bell(engine
    //     .output("tip")
    //     .unwrap(), "generous".to_string(), .., 5.0, 2.0, 3.0);
    // // tip.add_term(cheap);
    // // tip.add_term(average);
    // // tip.add_term(generous);

    // let mut rule_block = RuleBlock::new(Norm {}, Norm {}, Norm {});

    let expression = Proposition {
        hedges: Default::default(),
        term: Rc::downgrade(&poor),
        // variable: engine.input("service").unwrap(),
    };
    let antecedent = Antecedent::new(expression);
    // let consequent = Consequent {};

    let rule = Rule::new(antecedent, 0f64);
    println!("{}", rule.to_string());
    // rule_block.add_rule(rule);

    // engine.register_rule_block(rule_block);
}
