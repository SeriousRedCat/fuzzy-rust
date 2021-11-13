use fuzzy;
use fuzzy::antecedent::Antecedent;
use fuzzy::consequent::Consequent;
use fuzzy::expression::Proposition;
use fuzzy::hedge::Hedge;
use fuzzy::norm::Norm;
use fuzzy::rule::Rule;
use fuzzy::rule_block::{RuleBlock, RuleBlockTrait};
use fuzzy::term::TermTrait;
use fuzzy::variable::VariableTrait;

fn main() {
    let mut service = fuzzy::variable::Variable::new("service".to_string());
    let poor = fuzzy::term::bell("poor".to_string(), .., 5.0, 2.0, 3.0);
    let good = fuzzy::term::bell("good".to_string(), .., 5.0, 2.0, 3.0);
    let excelent = fuzzy::term::bell("excelent".to_string(), .., 5.0, 2.0, 3.0);

    service.add_term(poor);
    service.add_term(good);
    service.add_term(excelent);

    //     for i in 0..10 {
    //     println!("Poor {}", poor.membership(i.into()));
    // }

    let mut food = fuzzy::variable::Variable::new("food".to_string());
    let rancid = fuzzy::term::bell("rancid".to_string(), .., 5.0, 2.0, 3.0);
    let delicious = fuzzy::term::bell("delicious".to_string(), .., 5.0, 2.0, 3.0);

    food.add_term(rancid);
    food.add_term(delicious);

    let mut tip = fuzzy::variable::Variable::new("tip".to_string());
    let cheap = fuzzy::term::bell("cheap".to_string(), .., 5.0, 2.0, 3.0);
    let average = fuzzy::term::bell("average".to_string(), .., 5.0, 2.0, 3.0);
    let generous = fuzzy::term::bell("generous".to_string(), .., 5.0, 2.0, 3.0);
    tip.add_term(cheap);
    tip.add_term(average);
    tip.add_term(generous);

    let mut engine = fuzzy::engine::Engine::new();
    engine.register_input("service", service);
    engine.register_input("food", food);
    engine.register_output("tip", tip);

    let mut rule_block = RuleBlock::new(Norm {}, Norm {}, Norm {});

    let _expression = Proposition::<Hedge> {
        hedges: Default::default(),
        term: engine
            .input("service")
            .unwrap()
            .term(&"poor".to_string())
            .unwrap(),
        // variable: &service,
    };
    let antecedent = Antecedent::new(1f64);
    let consequent = Consequent {};

    let rule = Rule::new(antecedent, consequent);
    rule_block.add_rule(rule);

    engine.register_rule_block(rule_block);
}
