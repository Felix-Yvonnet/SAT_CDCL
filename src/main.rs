mod parser;
mod all_types;
mod solver;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let cnf = parser::parse_cnf(&args[0]).unwrap();
    let mut solver = solver::Solver::new(cnf);
    solver.solve(Some(std::time::Duration::from_secs(10)));
    match solver.status {
        None => {
            eprintln!("Time duration exceeded")
        },
        Some(satisfiable) => {
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
    use super::all_types::*;

    use walkdir::WalkDir;
    fn sat_model_check(clauses: &[Vec<all_types::Lit>], assigns: &[Option<bool>]) -> bool {
        for clause in clauses.iter() {
            let mut satisfied = false;
            for lit in clause {
                match assigns[lit.var().0 as usize] {
                    Some(true) => {
                        if lit.is_pos() {
                            satisfied = true;
                            break;
                        }
                    }
                    Some(false) => {
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

    fn clauses_to_cnf(clauses: &[Vec<Lit>], output_file_name: &str) -> std::io::Result<()> {
        use std::io::prelude::*;

        let mut f = std::fs::File::create(output_file_name)?;
        let mut var_num = 0;
        clauses.iter().for_each(|clause| {
            for c in clause.iter() {
                var_num = std::cmp::max(var_num, c.var().0 + 1);
            }
        });
        writeln!(f, "p cnf {} {}", var_num, clauses.len())?;
        for clause in clauses.iter() {
            let line = clause
                .iter()
                .enumerate()
                .map(|(i, x)| {
                    let v = if x.is_pos() {
                        format!("{}", x.var().0 + 1)
                    } else {
                        format!("-{}", x.var().0 + 1)
                    };
                    if i == clause.len() - 1 {
                        format!("{} 0", v)
                    } else {
                        format!("{} ", v)
                    }
                })
                .collect::<String>();
            writeln!(f, "{}", line)?;
        }
        Ok(())
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
        let entries = WalkDir::new(format!("cnf/{}/", which));
        for entry in entries
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| !e.file_type().is_dir())
        {
            let path_str = entry.path().to_str().unwrap();

            if path_str.ends_with(".cnf") {
                let cnf = parse_cnf(path_str).unwrap();
                let mut solver = Solver::default();
                cnf.clauses
                    .iter()
                    .for_each(|clause| solver.add_clause(all_types::Clause { clause: clause }));

                eprintln!("Solving... {}", path_str);
                let status = solver.solve(Some(std::time::Duration::from_secs(10)));
                assert!(solver.status == status);

                match status {
                    None => {
                        eprintln!("Too much time for this one: {}", path_str);
                        continue;
                    },
                    Some(satisfiable) => {
                        if satisfiable == expected {
                            if !sat_model_check(&cnf.clauses, &solver.models) {
                                eprintln!(
                                    "Failed in my code T_T cnf: {}, Result: {:?} Expected: {:?}",
                                    path_str, status, expected
                                );
                                assert!(false);
                            }
                        } else {
                            eprintln!(
                                "Mismatch cnf: {}, Result: {:?} Expected: {:?}",
                                path_str, status, expected
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

