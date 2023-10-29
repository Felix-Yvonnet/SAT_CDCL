use crate::all_types::*;
use std::time::Duration;

pub trait Solver {
    fn new(clauses : &mut Cnf) -> Self;
    fn specific_solve(&mut self, max_time : Option<Duration>) -> (Option<bool>, Duration);
    fn assigns(&self) -> &Vec<BoolValue>;
    
    fn solve(&mut self, cnfs: Vec<Cnf>, max_time: Option<std::time::Duration>) {
        for cnf in cnfs.iter_mut() {
            println!("Solving ...");
            let (is_sat, time_spent) = self.specific_solve(max_time);
            if is_sat.is_none() {
                println!("Time duration exceeded");
            } else {
                println!(
                    "Solved in {} sec and obtained : {}",
                    time_spent.as_secs_f64(),
                    if is_sat.unwrap() {
                        "\x1b[32mSAT\x1b[0m"
                    } else {
                        "\x1b[31mUNSAT\x1b[0m"
                    }
                );
            }
        }
    }
}