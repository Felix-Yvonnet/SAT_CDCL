use crate::*;
use std::time::{Duration, Instant};

#[derive(Debug, Default)]
pub struct Solver {
    // The clauses (initial and added ones)
    clauses: AllClauses,
    // Assignments of vars and decision levels of the assignments
    // and the implication graph
    working_model: WorkingModel,
    // Wether it is sat or not
    pub status: Option<bool>,
    level: usize,
}

impl Solver {
    pub fn new(mut clauses: CNF) -> Self {
        let n = clauses.var_num;
        let mut solver = Solver {
            clauses: AllClauses { clauses: vec![] },
            working_model: WorkingModel::new(n),
            status: None,
            level: 0,
        };
        clauses.iter().for_each(|clause| {
            solver.add_clause(clause.to_vec());
        });
        solver
    }

    pub fn add_clause(&mut self, clause: Clause) {
        if clause.is_empty() {
            self.status = Some(false);
        }else if clause.len() == 1 {
            let lit = clause[0];
            self.working_model.assign(
                lit.get_var(),
                BoolValue::from(lit.is_neg() as i8),
                self.level,
                0,
            )
        }
        else {
            self.clauses.push(clause);
        }
    }

    // Implement the CDCL algorithm
    fn cdcl(&mut self, maxtime: Option<Duration>) -> Duration {
        let start = Instant::now();
        self.propagate();

        while self.working_model.state_formula(&self.clauses) != BoolValue::True {
            if let Some(time) = maxtime {
                if start.elapsed() > time {
                    self.status = None;
                    return start.elapsed();
                }
            }
            while self.working_model.state_formula(&self.clauses) == BoolValue::False {
                // println!("State of the formula is false");
                if self.level == 0 {
                    self.status = Some(false);
                    return start.elapsed();
                }
                let (lvl, learnt) = self.analyze_conflict();
                println!("adding clause :");
                for lit in &learnt {
                    println!("    {} {:?}", lit.is_pos(), lit.get_var())
                }
                self.backtrack(lvl as usize);
                self.add_clause(learnt);
                self.propagate();
                // println!("ended backtracking and propagation");
            }
            if self.working_model.state_formula(&self.clauses) == BoolValue::Undefined {
                // println!("State of the formula is undefined");
                self.level += 1;
                self.decide();
                self.propagate();
            }
        }
        // println!("state of the formula is true... ending");
        self.status = Some(true);
        start.elapsed()
    }

    pub fn solve(&mut self, maxtime: Option<Duration>) -> Duration {
        if self.status.is_some() {
            return Duration::from_secs(0);
        }
        println!("Solving...");
        self.cdcl(maxtime)
    }

    // Implement the decision phase of CDCL
    fn decide(&mut self) {
        println!("deciding {:?} to 1 at level {}", self.working_model.next_unassigned(), self.level);
        // TODO
        // use random_unassigned for random variable
        // and assigns a random bool
        self.working_model.assign(
            self.working_model.next_unassigned(),
            BoolValue::True,
            self.level,
            0,
        )
    }

    // Implement clause propagation
    fn propagate(&mut self) {
        let mut something_was_done: bool = true;
        let mut index_number: usize = 1;

        while something_was_done {
            something_was_done = false;

            for clause in self.clauses.clauses.iter() {
                if self.working_model.is_unit_clause(clause).is_some() {

                    something_was_done = true;
                    index_number += 1;

                    let to_be_set_true = self.working_model.is_unit_clause(clause).unwrap();
                    // println!("    unit propagation sets {:?} to {:?}", to_be_set_true.get_var(), BoolValue::from(to_be_set_true.is_neg() as i8) );
                    self.working_model.assign(
                        to_be_set_true.get_var(),
                        BoolValue::from(to_be_set_true.is_neg() as i8),
                        self.level,
                        index_number,
                    );

                    self.working_model
                        .add_implications(to_be_set_true.get_var(), clause.clone())
                }
            }
        }
    }

    // Implement conflict resolution and clause learning
    fn analyze_conflict(&mut self) -> (i32, Clause) {
        let conflict = self.working_model.conflicting(&self.clauses);
        let conflict_clause = self.working_model.find_conflict(&conflict.unwrap());
        // find decision level to backtrack to
        // it is the maximum of all the decision levels in conflict clause - 1
        let mut max = self.working_model.level(conflict_clause[0].get_var());
        for lit in &conflict_clause {
            if self.working_model.level(lit.get_var()) > max {
                max = self.working_model.level(lit.get_var())
            }
        }
        (max as i32 - 1, conflict_clause)
    }

    fn backtrack(&mut self, level: usize) {
        println!("backtracking to {}", level);
        self.level = level;
        self.working_model.backtracking(level);
    }

    /* 
    pub fn assigns(&self) -> Vec<BoolValue> {
        self.working_model.get_assigned().clone()
    }
    */
}
