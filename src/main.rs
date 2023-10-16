mod parser;
mod all_types;
mod solver;
mod tautosolver;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let cnf = parser::parse_cnf(&args[1]).unwrap();
    let mut slowsolver = tautosolver::TautoSolver::new(cnf.var_num, cnf.clauses.clone());
    let mut solver = solver::Solver::new(cnf);
    let time_spent = solver.solve(Some(std::time::Duration::from_secs(10))).as_millis();
    let is_sat = slowsolver.solve();
    println!("{}", if is_sat {"SAT"}  else {"UNSAT"});
    match solver.status {
        None => {
            eprintln!("Time duration exceeded")
        },
        Some(satisfiable) => {
            println!("Computed in {time_spent} seconds.");
            if satisfiable {
                println!("Satisfiable");
                println!("{:?}", solver.models)
            } else {
                println!(
                    "Unsatisfiable"
                )
            }
        },
    }
    return;
}



#[cfg(test)]
mod tests {
    use std::process::exit;

    use super::parser::*;
    use super::solver::Solver;

    use walkdir::WalkDir;
    fn sat_model_check(clauses: &[Vec<all_types::Lit>], assigns: &[all_types::BoolValue]) -> bool {
        for clause in clauses.iter() {
            let mut satisfied = false;
            for lit in clause {
                match assigns[lit.get_var().0 as usize] {
                    all_types::BoolValue::True => {
                        if lit.is_pos() {
                            satisfied = true;
                            break;
                        }
                    }
                    all_types::BoolValue::False => {
                        if lit.is_neg() {
                            satisfied = true;
                            break;
                        }
                    }
                    _ => {}
                };
            }
            if !satisfied {
                return false;
            }
        }
        true
    }
    fn test_all_files(which: &str) {
        let expected = match which {
            "sat" => true,
            "unsat" => false,
            _ => {
                println!("Expected \"sat\" or \"unsat\" but got \"{which}\"");
                exit(1);
            }
            
        }; 
        let entries = WalkDir::new(format!("tests/{}/", which));
        for entry in entries
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| !e.file_type().is_dir())
        {
            let path_str = entry.path().to_str().unwrap();

            if path_str.ends_with(".cnf") {
                let cnf = parse_cnf(path_str).unwrap();
                let tmp_clauses = cnf.clauses.clone();
                let mut solver = Solver::new(cnf);
                eprintln!("Solving... {}", path_str);
                solver.solve(Some(std::time::Duration::from_secs(10)));
                let status = solver.status;

                match status {
                    None => {
                        eprintln!("Too much time for this one: {}", path_str);
                        continue;
                    },
                    Some(satisfiable) => {
                        // let model = Some(self.models.iter().map(|&opt| opt.unwrap()).collect())
                        if satisfiable == expected {
                            if !sat_model_check(&tmp_clauses, &solver.models) {
                                eprintln!(
                                    "Failed in my code T_T cnf: {}, Result: {:?} Expected: {:?}",
                                    path_str, satisfiable, expected
                                );
                                assert!(false);
                            }
                        } else {
                            eprintln!(
                                "Mismatch cnf: {}, Result: {:?} Expected: {:?}",
                                path_str, satisfiable, expected
                            );
                            assert!(false);
                        }
                    },
                }
            }
        }
    }

    #[test]
    fn test_solve() {
        test_all_files("sat");
        test_all_files("unsat");
    }
}

