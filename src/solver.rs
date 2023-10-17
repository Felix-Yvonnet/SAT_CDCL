use std::time::{Duration, Instant};


#[derive(Debug, Default)]
pub struct Solver {
    // The number of initial variables
    n: usize,
    // The clauses (initial and added ones)
    clauses: all_types::AllClauses,
    working_model: all_types::WorkingModel,
    // The 2 watched litterals (optimize later)
    watched: Vec<Vec<usize>>,
    // The final assignments that do satisfy the problem
    pub models: Vec<all_types::BoolValue>,
    // Wether it is sat or not
    pub status: Option<bool>,
    level: usize,
}

impl Solver {
    pub fn new(mut clauses: all_types::CNF) -> Solver {
        let n = clauses.var_num;
        let mut solver = Solver {
            n,
            clauses: all_types::AllClauses { clauses: clauses.clauses.clone() },
            working_model: all_types::WorkingModel::new(n),
            models: vec![],
            watched: vec![vec![]; 2 * n],
            status: None,
            level: 0,
        };
        clauses.iter().for_each(|clause| {
            if clause.len() == 1 {
                solver.working_model.assign(
                    clause[0].get_var(),
                     all_types::BoolValue::True, 
                     0, 
                     0);
            } else {
                solver.add_clause(clause.to_vec());
            }
        });
        solver
    }

    pub fn add_clause(&mut self, clause: all_types::Clause) {
        if clause.is_empty() {
            self.status = Some(false);
        } else if clause.len() == 1 {
            let lit = clause[0];
            self.working_model.assign(
                lit.get_var(),
                all_types::BoolValue::from(lit.is_neg() as i8), 
                self.level,
                0,
            )
        } else {
            self.clauses.push(clause);
        }
    }

    // Implement the CDCL algorithm
    fn cdcl(&mut self, maxtime: Option<Duration>) -> Duration {
        // Implement the main CDCL loop here
        // You'll need to track variables, propagate, make decisions, and backtrack
        // You can implement conflict analysis and clause learning here
        // Return true if a solution is found and false if the formula is unsatisfiable
        if let Some(time) = maxtime {
            let start = Instant::now();
            while !self.working_model.all_assigned() {
                if start.elapsed() > time {
                    self.status = None;
                    return start.elapsed();
                }
                let conflict_clause = self.propagate();
                if let Some(conflict) = conflict_clause {
                    // We found a conflict
                    let (lvl, learnt) = self.analyze_conflict(conflict);
                    if lvl == 0 {
                        self.status = Some(false);
                        return start.elapsed();
                    }
                    self.add_clause(learnt);
                    self.backtrack(lvl as usize);
                } else if self.working_model.all_assigned() {
                    break;
                } else {
                    self.level += 1;
                    self.decide();
                }
            }
            self.models = self.working_model.get_assigned();
            let mut is_sat = true;
            for clause in self.clauses.clauses.iter() {
                let mut is_verified = false;
                for lit in clause.iter() {
                    match self.working_model.eval(*lit) {
                        all_types::BoolValue::False => {},
                        all_types::BoolValue::True => {
                            is_verified = true;
                            break
                        },
                        all_types::BoolValue::Undefined => {
                            self.status = Some(false);
                            return start.elapsed();
                        }
                    }
                }
                if !is_verified {
                    is_sat = false;
                    break
                }

            }
            self.status = Some(is_sat);
            start.elapsed()

        } else {
            loop {
                unimplemented!();
            }
        }
    }

    pub fn solve(&mut self, maxtime: Option<Duration>) -> Duration {
        if let Some(_) = self.status {
            return Duration::from_secs(0);
        }
        println!("Solving...");
        self.cdcl(maxtime)
    }

    // Implement the decision phase of CDCL
    fn decide(&mut self) {
        // Implement variable decision heuristic (e.g., VSIDS, random, etc.)
        // Assign the chosen variable
        println!("deciding {:?} to {:?} with dl {:?}", self.working_model.next_unassigned(), all_types::BoolValue::True, self.level);
        self.working_model.assign(
            self.working_model.next_unassigned(), 
            all_types::BoolValue::True,
             self.level,
             0,
        )
    }

    // Implement clause propagation
    fn propagate(&mut self) -> Option<all_types::Clause> {
        // Implement unit clause propagation and conflict detection
        // Return true if no conflicts are found, and false if a conflict is detected
        let mut something_was_done: bool = true;
        let mut indice_number: usize = 1;
        while something_was_done {
            something_was_done = false;
            for clause in self.clauses.clauses.iter() {
                let mut last_unknown: Option<all_types::Lit> = None;
                let mut multiple_seen: bool = false;
                let mut some_unset: bool = false;
                let mut is_satisfied: bool = false;
                for lit in clause.iter() {
                    match self.working_model.eval(*lit) {
                        all_types::BoolValue::Undefined => {
                            some_unset = true;
                            if let Some(_) = last_unknown {
                                multiple_seen = true;
                                break
                            } else {
                                last_unknown = Some(*lit);
                            }
                        },
                        all_types::BoolValue::True => is_satisfied = true,
                        _ => (),
                    }
                }
                if !some_unset && !is_satisfied {
                    return Some(clause.clone());
                }
                if !multiple_seen && some_unset {
                    indice_number += 1;
                    let to_be_set_true = last_unknown.unwrap();                    
                    something_was_done = true;
                    self.working_model.assign(
                        to_be_set_true.get_var(),
                        all_types::BoolValue::from(to_be_set_true.is_neg() as i8),
                        self.level,
                        indice_number,
                    );
                    if self.level == 0 {
                    }
                }
            }
        }
        None
    }

    // Implement conflict resolution and clause learning
    fn analyze_conflict(&mut self, conflict: all_types::Clause) -> (i32, all_types::Clause) {

        // Implement conflict analysis and clause learning
        let mut max = conflict[0].get_var();
        // Find the last changed : it should be the one with problems
        for lit in conflict.iter() {
            if self.working_model.precise_level(lit.get_var()) > self.working_model.precise_level(max) {
                max = lit.get_var()
            }
        }

        // Just add the other without checking anything else
        let mut new_clause = vec![];
        for lit in conflict.iter() {
            if lit.get_var() != max {
                new_clause.push(!*lit)
            }
        }

        (self.working_model.level(max) as i32 - 1, new_clause)
    }

    fn backtrack(&mut self, level: usize) {
        self.level = level;
        self.working_model.backtracking(level);
    }
}