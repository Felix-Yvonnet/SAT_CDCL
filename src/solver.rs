use std::{time::Duration, process::exit};


#[derive(Debug, Default)]
pub struct Solver<'a> {
    n: usize,
    clauses: Vec<all_types::Clause::<'a>>,
    pub models: Vec<Option<bool>>,
    assigns: all_types::AssignData,
    watched: Vec<Vec<usize>>,
    pub status: Option<bool>,
}

impl Solver<'static> {
    pub fn new(clauses: all_types::CNF) -> Solver<'static> {
        let n = match clauses.var_num {
            None => {
                eprintln!("No variable in the clause.");
                exit(2);
            }
            Some(y) => y,
        };
        let mut solver = Solver {
            n,
            clauses: vec![],
            models: vec![None; n],
            assigns: all_types::AssignData::default(),
            watched: vec![vec![]; 2 * n],
            status: None,
        };
        clauses.clauses.iter().for_each(|clause| {
            if clause.len() == 1 {
                solver.assigns.enqueue(clause[0]);
            } else {
                solver.add_clause(all_types::Clause { clause:clause});
            }
        });
        solver
    }

    pub fn add_clause(&mut self, clause: all_types::Clause) {
        todo!()
    }
    pub fn solve(&mut self, maxtime: Option<Duration>)-> Option<bool> {
        todo!()
    }
}