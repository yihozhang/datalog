use crate::ast::*;
use crate::rel::*;
use std::collections::{HashMap, HashSet};

// trait EvalStrategy {
//     fn run<'a, R>(
//         &self,
//         rules: &Vec<Rule>,
//         schemas: &'a HashMap<Symbol, Schema>,
//         rels: &mut HashMap<Symbol, R>,
//     ) where
//         R: Rel<'a> + 'a;
// }

struct SemiNaiveEval {}

impl SemiNaiveEval {
    fn eval_approx_delta<'a, Ra: Rel<'a>>(
        rules: &Vec<Rule>,
        schemas: &HashMap<Symbol, Schema>,
        delta_rels: &'a HashMap<Symbol, Ra>,
        rels: &HashMap<Symbol, Ra>,
    ) -> HashMap<Symbol, Ra>
    {
        let mut approx_delta_rels = HashMap::<Symbol, Ra>::new();

        for rule in rules {
            let head = &rule.head;
            let body = &rule.body;
            let mut var_table = HashMap::<Symbol, Cell>:: new();

            for delta_idx in 0..body.len() {
                delta_rels.get(&body[delta_idx].rel_id).map(|delta_rel| {

                    for delta_tuple in delta_rel.iter() {
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

    fn run<'a, Ra: Rel<'a> + 'a>(
        &self,
        rules: &Vec<Rule>,
        schemas: &HashMap<Symbol, Schema>,
        rels: &mut HashMap<Symbol, Ra>,
    )
    {
        let mut approx_delta_rels: HashMap<Symbol, Ra>;
        let mut delta_rels: HashMap<Symbol, Ra> = HashMap::new();

        for (sym, schema) in schemas.iter() {
            let edb = rels.entry(sym.clone()).or_insert(Ra::new(schema));
            delta_rels.insert(sym.clone(), edb.clone());
        }

        loop {
            approx_delta_rels =
                SemiNaiveEval::eval_approx_delta(rules, schemas, &delta_rels, &rels);
        }

        // let
        unimplemented!();
    }
}

// impl EvalStrategy for SemiNaiveEval {
//     // TODO: assume no negation/aggregate rules
//     // TODO: type-checking needed (variable should always refer to the same type)
//     // TODO: automatic index selection
    // fn run<'a, R>(
    //     &self,
    //     rules: &Vec<Rule>,
    //     schemas: &'a HashMap<Symbol, Schema>,
    //     rels: &mut HashMap<Symbol, R>,
    // ) where
    //     R: Rel<'a> + 'a,
    // {
    //     let mut approx_delta_rels: HashMap<Symbol, R>;
    //     let mut delta_rels: HashMap<Symbol, R> = HashMap::new();

    //     for (sym, schema) in schemas.iter() {
    //         let edb = rels.entry(sym.clone()).or_insert(R::new(schema));
    //         delta_rels.insert(sym.clone(), edb.clone());
    //     }

    //     loop {
    //         approx_delta_rels =
    //             SemiNaiveEval::eval_approx_delta(rules, schemas, &delta_rels, &rels);
    //     }

    //     // let
    //     unimplemented!();
    // }
// }
