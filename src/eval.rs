use crate::ast::*;
use crate::rel::*;
use std::collections::{HashMap, HashSet};

trait EvalStrategy {
    fn run<R>(&self, rules: &Vec<Rule>, schemas: &HashMap<Symbol, Schema>, rels: &mut HashMap<Symbol, R>)
    where
        R: Rel;
}

struct SemiNaiveEval { }

impl SemiNaiveEval {
}

impl EvalStrategy for SemiNaiveEval {
    // TODO: assume no negation/aggregate rules
    fn run<R>(&self, rules: &Vec<Rule>, schemas: &HashMap<Symbol, Schema>, rels: &mut HashMap<Symbol, R>)
    where
        R: Rel
    {
        
    }
}