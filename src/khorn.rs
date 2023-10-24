use crate::all_types::*;
use std::collections::hash_set::HashSet;

pub struct KhornSolver {
    num_var: usize,
    num_clauses: usize,
    clauses: CAllClauses,
    assigned_pos: Vec<Var>,
}

impl KhornSolver {
    pub fn new(mut clauses: CNF) -> Self {
        let mut new_clauses = vec![];
        for clause in clauses.iter() {
            new_clauses.push(CClause::new(clause.clone(), {
                let ind = clause.iter().position(|lit| lit.is_pos());
                if ind.is_none() {
                    None
                } else {
                    Some(clause[ind.unwrap()].get_var())
                }
            }));
        }
        KhornSolver { 
            num_var: clauses.var_num,
            num_clauses: clauses.cl_num,
            clauses: CAllClauses::new(new_clauses), 
            assigned_pos: vec![] 
        }
    }

    pub fn solve(&mut self) -> (bool, std::time::Duration) {        
        let start = std::time::Instant::now();
        (self.linear_solve(), start.elapsed())
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

    fn linear_solve(&mut self) -> bool {
        // ind(clause) = self.clauses.clauses.position(clause)
        let mut score: Vec<u32> = vec![0; self.num_clauses]; // ind(clause) -> score
        let mut clauses_with_negvar: Vec<HashSet<u32>> = vec![HashSet::new(); self.num_var]; // var -> list[ind(clause)]
        assert!(self.clauses.clauses.len() == self.num_clauses);
        for k in 0..self.clauses.clauses.len() {
            for lit in self.clauses.clauses[k].iter() {
                score[k] += lit.is_neg() as u32;
                if lit.is_neg() {
                    clauses_with_negvar[lit.get_var()].insert(k as u32);
                }
            }

        }
        let max_score = *score.iter().max().unwrap();
        let mut pool: Vec<HashSet<u32>> = vec![HashSet::new(); (max_score+1) as usize]; // score -> list[ind(clause)]
        for k in 0.. self.num_clauses {
            pool[score[k] as usize].insert(k as u32);
        }

        let mut solutions: HashSet<Var> = HashSet::new();
        while !pool[0].is_empty() {
            let curr = *pool[0].iter().next().unwrap();
            pool[0].remove(&curr);
            let curr_clause = &self.clauses.clauses[curr as usize];
            let opt_v = curr_clause.pos;
            if opt_v.is_none() {
                return false;
            }
            let v = opt_v.unwrap();
            if solutions.contains(&v) || clauses_with_negvar[v].contains(&curr) {
                continue
            }
            solutions.insert(v);
            for &c in clauses_with_negvar[v].iter() {
                pool[score[c as usize] as usize].remove(&c);
                score[c as usize] -= 1;
                pool[score[c as usize] as usize].insert(c);
            }
        }
        true
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

    pub fn assigns(&self) -> Vec<BoolValue> {
        let mut assigns = vec![BoolValue::False; self.num_var];
        for var in self.assigned_pos.iter() {
            assigns[*var] = BoolValue::True;
        }
        assigns

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