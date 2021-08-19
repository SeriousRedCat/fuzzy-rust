use fuzzy;
use fuzzy::term::Term;

fn main() {
    let term = fuzzy::term::bell(.., 5.0, 2.0, 3.0);
    for i in 0..10 {
        println!("Bell {}", term.value(i.into()));
    }
}
