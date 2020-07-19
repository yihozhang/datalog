use crate::ast::*;
use crate::rel::*;
use std::collections::HashMap;

trait EvalStrategy {
    fn run<R>(rules: Vec<Rule>, edb: HashMap<Symbol, R>)
    where
        R: Rel;
}
