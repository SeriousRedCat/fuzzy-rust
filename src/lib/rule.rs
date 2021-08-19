
trait Rule {
    fn activation_degree() -> f64;
}

struct RuleImpl {

}

impl Rule for RuleImpl {
    fn activation_degree() -> f64 {
        0.0
    }
}