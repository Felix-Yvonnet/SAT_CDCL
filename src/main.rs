mod all_types;
mod khorn;
mod parser;
mod sat2;
mod solver;
mod tautosolver;

use crate::all_types::*;

fn get_args(args: Vec<String>) -> (Vec<String>, Vec<String>) {
    let mut flags = vec![];
    let mut files = vec![];
    let mut i = 1;
    while i < args.len() {
        if args[i].starts_with('-') {
            if args[i] == "-t" || args[i] == "--time" {
                i += 1;
                if i == args.len() {
                    eprintln!("Expected a time after the \"--time\" of \"-t\" argument");
                    std::process::exit(6);
                } else if args[i].parse::<u8>().is_ok() {
                    flags.push(args[i].to_string());
                } else {
                    eprintln!(
                        "Expected a number after the \"--time\" of \"-t\" argument, got {}",
                        args[i]
                    );
                    std::process::exit(6);
                }
            }
            flags.push(args[i].to_string());
        } else {
            files.push(args[i].to_string());
        }

        i += 1;
    }
    (flags, files)
}

fn get_cnfs(files: Vec<String>) -> Vec<CNF> {
    let mut cnfs: Vec<CNF> = Vec::new();
    for file in files {
        let cnf: crate::CNF = parser::parse_cnf(&file).unwrap();
        cnfs.push(cnf);
    }
    cnfs
}

fn quick_solver(cnfs: Vec<CNF>, max_time: Option<std::time::Duration>, verbose: bool) {
    for cnf in cnfs {
        let mut solver = solver::Solver::new(cnf);
        let time_spent = solver.solve(max_time).as_secs_f64();
        if solver.status.is_none() {
            println!("Time duration exceeded");
        } else {
            println!(
                "Solved and obtained : {}",
                if solver.status.unwrap() {
                    "\x1b[32mSAT\x1b[0m"
                } else {
                    "\x1b[31mUNSAT\x1b[0m"
                }
            );
            if verbose {
                println!("Solved in {time_spent} seconds");
            }
        }
    }
}

fn khorn_solver(cnfs: Vec<CNF>, _max_time: Option<std::time::Duration>, verbose: bool) {
    for cnf in cnfs {
        println!("Solving...");
        let mut solver = khorn::KhornSolver::new(cnf);
        let (is_sat, time_spent) = solver.solve();
        println!(
            "Solved and obtained : {}",
            if is_sat {
                "\x1b[32mSAT\x1b[0m"
            } else {
                "\x1b[31mUNSAT\x1b[0m"
            }
        );
        if verbose {
            println!("Solved in {} seconds", time_spent.as_secs_f64());
        }
    }
}

fn dummy_solver(cnfs: Vec<CNF>, max_time: Option<std::time::Duration>, verbose: bool) {
    let mut mean_time = 0.;
    let mut total_count = 0;
    for cnf in cnfs {
        let mut solver = tautosolver::TautoSolver::new(cnf);
        let (is_sat, time_spent) = solver.solve(max_time);

        if is_sat.is_none() {
            println!("Time duration exceeded");
        } else {
            println!(
                "Solved and obtained : {}",
                if is_sat.unwrap() {
                    "\x1b[32mSAT\x1b[0m"
                } else {
                    "\x1b[31mUNSAT\x1b[0m"
                }
            );
            if verbose {
                println!("Solved in {} seconds", time_spent.as_secs_f64());
            }
            total_count += 1;
            mean_time += time_spent.as_secs_f64();
        }
    }
    println!(
        "Mean time spent: {:.1$} seconds",
        mean_time / total_count as f64,
        6
    )
}

fn sat2_solver(cnfs: Vec<CNF>, _max_time: Option<std::time::Duration>, verbose: bool) {
    for cnf in cnfs {
        let start = std::time::Instant::now();
        let mut solver = sat2::SAT2::new(cnf.clone());
        let is_sat = solver.solve();
        let time_spent = start.elapsed();
        println!(
            "Solved in {} sec and obtained : {}",
            time_spent.as_secs_f64(),
            if is_sat {
                "\x1b[32mSAT\x1b[0m"
            } else {
                "\x1b[31mUNSAT\x1b[0m"
            }
        );
        if verbose {
            println!("No time for this one yet");
        }
        let assigns = solver.assigns;
        if is_sat && !sat_model_check(cnf.clauses.as_slice(), &assigns) {
            println!("Fuck off!!!");
        }
    }
}

fn help() {
    println!("This function imlements different SAT solvers.");
    println!("To use it, you may write `sat_solver [args] file(s)`");
    println!("Default is an optimization that determines which solver to use.");
    println!();
    println!("-h --help     Show this message");
    println!("--cdcl        Using the CDCL solver");
    println!("--khorn       Using the Khorn solver");
    println!("--dummy       Using the dummy solver");
    println!("-v --verbose  Print the model and different informations");
    println!("-t --time     Limit the maximum time (in seconds) to spend searching");
    println!("For example `./sat_solver --cdcl tests/sat/horn1.cnf tests/unsat/php6-4.cnf` will returns :");
    println!("SAT\nUNSAT");
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let (flags, files) = get_args(args);

    if flags.contains(&"-h".to_string()) || flags.contains(&"--help".to_string()) {
        help();
        std::process::exit(0);
    }
    if files.is_empty() {
        eprintln!("Please provide at least one file to resolve");
        std::process::exit(5)
    }

    let cnfs = get_cnfs(files);
    let max_time = {
        let time = flags.iter().find(|&f| f.parse::<u64>().is_ok());
        time.map(|t| std::time::Duration::from_secs(
            t.parse::<u64>().ok().unwrap(),
        ))
    };

    let verbose = flags.contains(&"-v".to_string()) || flags.contains(&"--verbose".to_string());

    if !flags
        .iter()
        .any(|f| f.starts_with('-') && !(f.starts_with("-v")) && !(f.starts_with("--v")))
    {
        // No flags (other than the timer or verbose) are set
        for cnf in cnfs.clone() {
            if sat2::is_2sat(&cnf) {
                sat2_solver(vec![cnf], max_time, verbose);
            } else if khorn::is_khorn(&cnf) {
                khorn_solver(vec![cnf], max_time, verbose);
            } else {
                quick_solver(vec![cnf], max_time, verbose)
            }
        }
    }
    if flags.contains(&"--cdcl".to_string()) {
        quick_solver(cnfs.clone(), max_time, verbose);
    }
    if flags.contains(&"--khorn".to_string()) {
        if verbose && !khorn::is_khorn(&cnfs[0]) {
            println!("\x1b[31mNot a Horn\x1b[0m configuration but go on")
        }
        khorn_solver(cnfs.clone(), max_time, verbose);
    }
    if flags.contains(&"--dummy".to_string()) {
        dummy_solver(cnfs.clone(), max_time, verbose);
    }
    if flags.contains(&"--2sat".to_string()) {
        sat2_solver(cnfs, max_time, verbose);
    }
}

fn sat_model_check(clauses: &[Vec<Lit>], assigns: &[BoolValue]) -> bool {
    for clause in clauses.iter() {
        let mut satisfied = false;
        for lit in clause {
            match assigns[lit.get_var().0 as usize] {
                BoolValue::True => {
                    if lit.is_pos() {
                        satisfied = true;
                        break;
                    }
                }
                BoolValue::False => {
                    if lit.is_neg() {
                        satisfied = true;
                        break;
                    }
                }
                _ => println!("Some undefined value"),
            };
        }
        if !satisfied {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use std::process::exit;
    // use crate::tautosolver::TautoSolver;

    use super::*;

    use super::parser::*;
    use super::solver::Solver;

    use walkdir::WalkDir;

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
                solver.solve(Some(std::time::Duration::from_secs(1)));
                let status = solver.status;
                // let mut solver = TautoSolver::new(cnf.var_num, cnf.clauses);
                // let (status, time) = solver.solve(Some(std::time::Duration::from_secs(1)));

                match status {
                    None => {
                        eprintln!("\x1b[33mToo much time for this one\x1b[0m");
                        continue;
                    }
                    Some(satisfiable) => {
                        // let model = Some(self.models.iter().map(|&opt| opt.unwrap()).collect())
                        if satisfiable == expected {
                            if satisfiable
                                && !sat_model_check(tmp_clauses.as_slice(), &solver.assigns())
                            {
                                eprintln!(
                                    "Failed in my code T_T cnf: {}, Result: {:?} Expected: {:?}",
                                    path_str, satisfiable, expected
                                );
                                assert!(false);
                            } else {
                                eprintln!("\x1b[32mSuccess\x1b[0m")
                            }
                        } else {
                            eprintln!(
                                "Mismatch cnf: {}, Result: {:?} Expected: {:?}",
                                path_str, satisfiable, expected
                            );
                            assert!(false);
                        }
                    }
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
