use std::{time::{Duration, Instant}, f32::consts::E};

use all_types::WorkingModel;


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
    pub models: Vec<Option<bool>>,
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
                solver.models[clause[0].get_var()] = Some(true);
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
                all_types::BoolValue::from(lit.is_pos() as i8), 
                self.level,
            )
        } else {
            self.clauses.push(clause);
        }
    }

    // Implement the CDCL algorithm
    fn cdcl(&mut self, maxtime: Option<Duration>) -> Option<bool> {
        // Implement the main CDCL loop here
        // You'll need to track variables, propagate, make decisions, and backtrack
        // You can implement conflict analysis and clause learning here
        // Return true if a solution is found and false if the formula is unsatisfiable
        
        if let Some(time) = maxtime {
            let start = Instant::now();
            while !self.working_model.all_assigned() {
                if start.elapsed() > time {
                    self.status = None;
                    return None;
                }
                let conflict_clause = self.propagate();
                if let Some(conflict) = conflict_clause {
                    // We found a conflict
                    let (lvl, learnt) = self.analyze_conflict(conflict);
                    if lvl <= 0 {
                        return Some(false);
                    }
                    self.clauses.push(learnt);
                    self.backtrack(lvl);
                } else if self.working_model.all_assigned() {
                    return Some(true);
                } else {
                    self.level += 1;
                    self.decide();
                }
                
            }
            None

        } else {
            loop {
                unimplemented!();
            }
        }
    }

    pub fn solve(&mut self, maxtime: Option<Duration>) -> Option<bool> {
        if let Some(_) = self.status {
            return self.status
        }
        self.cdcl(maxtime)
    }

    // Implement the decision phase of CDCL
    fn decide(&mut self) {
        // Implement variable decision heuristic (e.g., VSIDS, random, etc.)
        // Assign the chosen variable
        self.working_model.assign(
            self.working_model.next_unassigned(), 
            all_types::BoolValue::True,
             self.level
        )
    }

    // Implement clause propagation
    fn propagate(&mut self) -> Option<all_types::Clause> {
        // Implement unit clause propagation and conflict detection
        // Return true if no conflicts are found, and false if a conflict is detected
        let mut something_was_done: bool = true;
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
                    let to_be_set_true: all_types::Lit = last_unknown.unwrap();                    
                    something_was_done = true;
                    self.working_model.assign(
                        to_be_set_true.get_var(),
                        all_types::BoolValue::True,
                        self.level
                    )
                }
            }
        }
        None
    }

    // Implement conflict resolution and clause learning
    fn analyze_conflict(&mut self, conflict: all_types::Clause) -> (usize, all_types::Clause) {
        // Implement conflict analysis and clause learning
        unimplemented!()
    }

    fn backtrack(&mut self, level: usize) {
        self.working_model.backtracking(level);
        self.level = level;
    }
}