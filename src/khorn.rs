use crate::all_types::*;

pub struct KhornSolver {
    clauses: CAllClauses,
    assigned_pos: Vec<Var>,
}

impl KhornSolver {
    pub fn new(mut clauses: CNF) -> Self {
        let mut new_clauses = vec![];
        for clause in clauses.iter() {
            new_clauses.push(CClause::new(clause.to_vec(), {
                let ind = clause.iter().position(|lit| lit.is_pos());
                if ind.is_none() {
                    None
                } else {
                    None
                }
            }));
        }
        KhornSolver { clauses: CAllClauses::new(new_clauses), assigned_pos: vec![] }
    }

    pub fn solve(&mut self) -> (bool, std::time::Duration) {        
        let start = std::time::Instant::now();
        (self.ssolve(), start.elapsed())
    }

    fn ssolve(&mut self) -> bool {
        loop {
            let shortest = self.get_shortest();
            if shortest.len() == 0 {
                return false;
            } else if shortest.len() > 1 {
                return true
            } else {
                let new_true = shortest.get_first();
                self.assigned_pos.push(new_true.get_var());
                for clause in self.clauses.iter() {
                    let ind = clause.iter().position(|&lit| lit.get_var() == new_true.get_var());
                    if let Some(val) = ind {
                        if clause.get_at_pos(val).is_pos() {
                            clause.is_present = false;
                        } else {
                            clause.decr_len();
                        }
                    }
                }
            }

        }
    }

    fn get_shortest(&self) -> &CClause {
        let mut current_min = &self.clauses.clauses[0];
        for clause in self.clauses.clauses.iter() {
            if clause.is_present {
                if clause.len() < current_min.len() || (clause.len() == current_min.len() && clause.pos.is_some()) {
                    current_min = clause
                }
            }
        }
        current_min
    }
}


pub fn is_khorn(cnf: &CNF) -> bool {
    for clause in cnf.clauses.iter() {
        let mut seen_pos = false;
        for lit in clause {
            if lit.is_pos() {
                if seen_pos {
                    return false;
                } else {
                    seen_pos = true;
                }

            }
        }
    }
    true
}