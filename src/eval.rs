use crate::ast::*;
use crate::rel::*;
use std::collections::{HashMap, HashSet};

trait EvalStrategy {
    fn run<R: Rel>(
        &self,
        rules: &Vec<Rule>,
        schemas: &HashMap<Symbol, Schema>,
        rels: &mut HashMap<Symbol, R>,
    );
}

struct SemiNaiveEval {}

impl SemiNaiveEval {
    fn eval_approx_delta<R: Rel>(
        rules: &Vec<Rule>,
        schemas: &HashMap<Symbol, Schema>,
        delta_rels: &HashMap<Symbol, R>,
        rels: &HashMap<Symbol, R>,
    ) -> HashMap<Symbol, R> {
        let mut approx_delta_rels = HashMap::<Symbol, R>::new();

        for rule in rules {
            let head = &rule.head;
            let body = &rule.body;

            for delta_idx in 0..body.len() {
                delta_rels.get(&body[delta_idx].rel_id).map(|delta_rel| {

                    let mut var_table = HashMap::<Symbol, (u32, Cell)>::new();
                    let schema = delta_rel.schema();

                    for delta_tuple in delta_rel.iter() {
                        schema.0.iter().zip(delta_tuple.iter()).for_each(|((sym, _ty, _sz), cell)| {
                            var_table.insert(sym.clone(), (1, cell));
                        });
                        
                        for idx in 0..body.len() {
                            if delta_idx != idx {
                                
                            }
                        }
                    }
                });
            }
        }

        unimplemented!();
    }
}

impl EvalStrategy for SemiNaiveEval {
    // TODO: assume no negation/aggregate rules
    // TODO: type-checking needed (variable should always refer to the same type)
    // TODO: automatic index selection
    fn run<R: Rel>(
        &self,
        rules: &Vec<Rule>,
        schemas: &HashMap<Symbol, Schema>,
        rels: &mut HashMap<Symbol, R>,
    ) {
        let mut approx_delta_rels: HashMap<Symbol, R>;
        let mut delta_rels: HashMap<Symbol, R> = HashMap::new();

        for (sym, schema) in schemas.iter() {
            let edb = rels.entry(sym.clone()).or_insert(R::new(schema));
            delta_rels.insert(sym.clone(), edb.clone());
        }

        loop {
            approx_delta_rels =
                SemiNaiveEval::eval_approx_delta(rules, schemas, &delta_rels, &rels);
        }

        unimplemented!();
    }
}
