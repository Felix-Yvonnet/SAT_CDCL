use crate::{all_types::*, solver::Solver};
use petgraph::graph::DiGraph;

/// A solver for 2SAT formulae.
/// A clause is said to be 2 SAT if each clause of the formula contains at most 2 literals.
/// This solver uses the tarjan algorithm to solve it linearly.
pub struct SAT2Solver {
    impl_graph: DiGraph<Lit, ()>,
    pub status: Option<bool>,
    pub assigns: Vec<BoolValue>,
}

impl Solver for SAT2Solver {
    fn new(cnf: &mut Cnf) -> SAT2Solver {
        if cnf.clauses.is_empty() {
            return SAT2Solver {
                impl_graph: DiGraph::new(),
                status: Some(true),
                assigns: vec![],
            };
        } else if cnf.clauses[0].is_empty() {
            return SAT2Solver {
                impl_graph: DiGraph::new(),
                status: Some(false),
                assigns: vec![],
            };
        }
        let mut impl_graph = DiGraph::new();
        let mut all_lits: Vec<Option<petgraph::stable_graph::NodeIndex>> =
            vec![None; 2 * cnf.var_num];
        for clause in cnf.clauses {
            let lit1 = clause[0];
            let lit2 = clause[1];
            if all_lits[lit1].is_none() {
                all_lits[lit1] = Some(impl_graph.add_node(lit1));
            }
            if all_lits[!lit1].is_none() {
                all_lits[!lit1] = Some(impl_graph.add_node(!lit1));
            }
            if all_lits[lit2].is_none() {
                all_lits[lit2] = Some(impl_graph.add_node(lit2));
            }
            if all_lits[!lit2].is_none() {
                all_lits[!lit2] = Some(impl_graph.add_node(!lit2));
            }

            impl_graph.add_edge(all_lits[!lit2].unwrap(), all_lits[lit1].unwrap(), ());
            impl_graph.add_edge(all_lits[!lit1].unwrap(), all_lits[lit2].unwrap(), ());
        }

        SAT2Solver {
            impl_graph,
            status: None,
            assigns: vec![BoolValue::Undefined; cnf.var_num],
        }
    }
    fn assigns(&self) -> &Vec<BoolValue> {
        panic!("not implemented for SAT2Solver")
    }
    fn specific_solve(&mut self, max_time : Option<std::time::Duration>) -> (Option<bool>, std::time::Duration) {
        let start = std::time::Instant::now();

        if self.status.is_some() {
            return (self.status, start.elapsed());
        };
        let sccs = petgraph::algo::tarjan_scc(&self.impl_graph);
        for scc in sccs {
            let mut all_literals = std::collections::HashSet::new();
            for node_lit in scc {
                let lit = self.impl_graph[node_lit];
                if all_literals.contains(&!lit) {
                    return (Some(false), start.elapsed());
                }
                all_literals.insert(lit);
                if self.assigns[lit.get_var()] == BoolValue::Undefined {
                    self.assigns[lit.get_var()] = BoolValue::from(lit.is_neg() as i8);
                }
            }
        }
        (Some(true), start.elapsed())
    }
}


pub fn is_2sat(cnf: &Cnf) -> bool {
    for clause in cnf.clauses.iter() {
        if clause.len() > 2 {
            return false;
        }
    }
    true
}
