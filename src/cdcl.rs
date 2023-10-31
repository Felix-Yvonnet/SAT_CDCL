use crate::*;
use rand::Rng;

/// A CDCL solver.
/// Clause Driven Conflict Learning is an algorithm that solves SAT in an amortized exponential time.
/// The amortized part allows us to be "efficient" on real input, that is to say that we postpone the exponential growth enough to make it usable.
#[derive(Debug)]
pub struct CdclSolver {
    // The clauses (initial and added ones)
    clauses: AllClauses,
    // Assignments of vars and decision levels of the assignments
    // and the implication graph
    working_model: WorkingModel,
    // Wether it is sat or not
    pub status: Option<bool>,
    level: usize,
    // implements 2 watch literals
    watchers: Watcher,
    // random generator
    rand: rand::rngs::ThreadRng,
}

impl<'a> solver::Solver<'a> for CdclSolver {
    fn new<'b: 'a>(clauses: &Cnf) -> Self {
        let n = clauses.var_num;
        let mut solver = CdclSolver {
            clauses: AllClauses { clauses: vec![] },
            working_model: WorkingModel::new(n),
            status: None,
            level: 0,
            watchers: Watcher::new(n),
            rand: rand::thread_rng(),
        };
        clauses.clauses.iter().for_each(|clause| {
            if clause.is_empty() {
                solver.status = Some(false);
            } else {
                solver.add_clause(clause.to_vec());
            }
        });
        let mut frequences = vec![0; n];
        clauses
            .clauses
            .iter()
            .for_each(|clause| clause.iter().for_each(|lit| frequences[lit.get_var()] += 1));
        solver
    }

    fn solve(&mut self) -> bool {
        if let Some(status) = self.status {
            return status;
        }
        self.cdcl()
    }

    fn assigns(&mut self) -> &Vec<BoolValue> {
        self.working_model.get_assigned()
    }
}

impl CdclSolver {
    pub fn add_clause(&mut self, clause: Clause) -> bool {
        if clause.len() == 1 {
            let lit = clause[0];
            if self.working_model.eval(lit) == BoolValue::False {
                self.status = Some(false);
                return false;
            }
            self.working_model.assign(
                lit.get_var(),
                BoolValue::from(lit.is_neg() as i8),
                self.level,
            )
        } else {
            let mut pos1 = clause[0];
            let mut pos2 = clause[1];
            let mut seen_one = false;
            for lit in clause.iter() {
                match self.working_model.eval(*lit) {
                    BoolValue::Undefined => {
                        if seen_one {
                            pos2 = *lit;
                        } else if self.working_model.eval(pos1) != BoolValue::True {
                            pos1 = *lit;
                            seen_one = true
                        }
                    }
                    BoolValue::True => {
                        pos1 = *lit;
                    }
                    _ => {}
                }
            }
            self.watchers.add(pos1, self.clauses.len());
            self.watchers.add(pos2, self.clauses.len());
            self.clauses.push(clause);
        }
        true
    }

    /// Implement the CDCL algorithm
    fn cdcl(&mut self) -> bool {
        self.propagate();

        loop {
            while self.working_model.state_formula(&self.clauses) == BoolValue::False {
                if self.level == 0 {
                    self.status = Some(false);
                    return false;
                }
                let (lvl, learnt) = self.analyze_conflict();
                self.backtrack(lvl as usize);
                if !self.add_clause(learnt) {
                    return false;
                }

                self.working_model.heap.decay_inc();

                self.propagate();
            }
            self.level += 1;
            if self.decide() {
                return true;
            }
            self.propagate();

            if self.working_model.state_formula(&self.clauses) == BoolValue::True {
                break;
            }
        }
        self.status = Some(true);
        true
    }

    /// Implement the decision phase of CDCL
    fn decide(&mut self) -> bool {
        loop {
            if let Some(var) = self.working_model.heap.pop() {
                if self.working_model.state_var(var) != BoolValue::Undefined {
                    continue;
                }
                self.working_model.assign(
                    var,
                    BoolValue::from(self.rand.gen_range(0..2)),
                    self.level,
                );
                return false;
            } else {
                self.status = Some(true);
                return true;
            }
        }
    }

    // Implement clause propagation
    fn propagate(&mut self) {
        let mut something_was_done: bool = true;

        while something_was_done {
            something_was_done = false;

            for clause in self.clauses.clauses.iter() {
                if let Some(to_be_set_true) = self.working_model.is_unit_clause(clause) {
                    something_was_done = true;
                    self.working_model.assign(
                        to_be_set_true.get_var(),
                        BoolValue::from(to_be_set_true.is_neg() as i8),
                        self.level,
                    );

                    self.working_model
                        .add_implications(to_be_set_true.get_var(), clause)
                }
            }
        }
    }

    // Implement conflict resolution and clause learning
    fn analyze_conflict(&mut self) -> (i32, Clause) {
        if let Some(conflict) = self.working_model.conflicting(&self.clauses) {
            let conflict_clause = self.working_model.find_conflict(&conflict);
            // find decision level to backtrack to
            // it is the maximum of all the decision levels in conflict clause - 1
            let mut max = self.working_model.level(conflict_clause[0].get_var());
            for lit in conflict_clause.iter() {
                let current = self.working_model.level(lit.get_var());
                if current > max {
                    max = current
                }
            }
            conflict_clause
                .iter()
                .for_each(|lit| self.working_model.heap.incr_activity(lit.get_var()));

            (max as i32 - 1, conflict_clause)
        } else {
            self::panic!("entered conflict analysis without a conflict")
        }
    }

    fn backtrack(&mut self, level: usize) {
        self.level = level;
        self.working_model.backtracking(level);
    }
}
