use crate::all_types::*;
use std::collections::hash_set::HashSet;

/// A solver for Horn formulae.
/// A clause is said to be a Horn clause if it contains at most one positive (non negated) literal. A Horn formulae is a conjunction of Horn clauses.
/// This solver is linear.
pub struct KhornSolver {
    num_var: usize,
    num_clauses: usize,
    clauses: CAllClauses,
    status: Option<bool>,
    assigned_pos: HashSet<Var>,
}

impl KhornSolver {
    pub fn new(mut clauses: CNF) -> Self {
        let mut status = None;
        let mut new_clauses = vec![];
        for clause in clauses.iter() {
            if clause.is_empty() {
                status = Some(false)
            } else {
                new_clauses.push(CClause::new(clause.clone(), {
                    let ind = clause.iter().position(|lit| lit.is_pos());
                    ind.map(|i| clause[i].get_var())
                }));
            }
        }
        KhornSolver {
            num_var: clauses.var_num,
            num_clauses: clauses.cl_num,
            status,
            clauses: CAllClauses::new(new_clauses),
            assigned_pos: HashSet::new(),
        }
    }

    pub fn solve(&mut self) -> (bool, std::time::Duration) {
        let start = std::time::Instant::now();
        if let Some(status) = self.status {
            (status, start.elapsed())
        } else {
            (self.linear_solve(), start.elapsed())
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
        let mut pool: Vec<HashSet<u32>> = vec![HashSet::new(); (max_score + 1) as usize]; // score -> list[ind(clause)]
        for k in 0..self.num_clauses {
            pool[score[k] as usize].insert(k as u32);
        }

        while !pool[0].is_empty() {
            let curr = *pool[0].iter().next().unwrap();
            pool[0].remove(&curr);
            let curr_clause = &self.clauses.clauses[curr as usize];
            let opt_v = curr_clause.pos;
            if opt_v.is_none() {
                return false;
            }
            let v = opt_v.unwrap();
            if self.assigned_pos.contains(&v) || clauses_with_negvar[v].contains(&curr) {
                continue;
            }
            self.assigned_pos.insert(v);
            for &c in clauses_with_negvar[v].iter() {
                pool[score[c as usize] as usize].remove(&c);
                score[c as usize] -= 1;
                pool[score[c as usize] as usize].insert(c);
            }
        }
        true
    }

    #[cfg(test)]
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
