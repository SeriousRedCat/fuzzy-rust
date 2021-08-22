use fuzzy;
use fuzzy::term::TermTrait;
use fuzzy::variable::VariableTrait;

fn main() {
    let mut input = fuzzy::variable::Variable::new("food".to_string());
    let term = fuzzy::term::bell(.., 5.0, 2.0, 3.0);

    for i in 0..10 {
        println!("Bell {}", term.value(i.into()));
    }

    input.add_term(term);

    let mut engine = fuzzy::engine::Engine::new();
    let i = engine.register_input(input);
    i.fuzzify(1.0);
}
