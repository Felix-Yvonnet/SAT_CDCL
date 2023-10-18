mod parser;
mod all_types;
mod solver;
mod tautosolver;
mod khorn;
use crate::all_types::*;

fn get_args(args: Vec<String>) -> (Vec<String>, Vec<String>) {
    let mut flags = vec![];
    let mut files = vec![];
    let mut i = 1;
    while i < args.len() {
        if args[i].starts_with('-') {
            if args[i] == "-t" || args[i] == "--time" {
                i+=1;
                if i == args.len() {
                    eprintln!("Expected a time after the \"--time\" of \"-t\" argument");
                    std::process::exit(6);
                } else if args[i].parse::<u8>().is_ok() {
                    flags.push(args[i].to_owned());
                } else {
                    eprintln!("Expected a number after the \"--time\" of \"-t\" argument, got {}", args[i]);
                    std::process::exit(6);
                }
            }
            flags.push(args[i].to_owned());
        } else {
            files.push(args[i].to_owned());
        }

        i+=1;
    }
    if files.is_empty() {
        eprintln!("Please provide at least one file to resolve");
        std::process::exit(5)
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
            println!("Solved and obtained : {}", if solver.status.unwrap() {"\x1b[32mSAT\x1b[0m"}  else {"\x1b[31mUNSAT\x1b[0m"});
            if verbose {
                println!("Solved in {} seconds", time_spent);
            }
        }
    }
}

fn khorn_solver(cnfs: Vec<CNF>, max_time: Option<std::time::Duration>, verbose: bool) {
    for cnf in cnfs {
        let mut solver = khorn::KhornSolver::new(cnf);
        let (is_sat, time_spent) = solver.solve();
        println!("Solved and obtained : {}", if is_sat {"\x1b[32mSAT\x1b[0m"}  else {"\x1b[31mUNSAT\x1b[0m"});
        if verbose {
            println!("Solved in {} seconds", time_spent.as_secs_f64());
        }
    }
}


fn dummy_solver(cnfs: Vec<CNF>, max_time: Option<std::time::Duration>, verbose: bool) {
    for cnf in cnfs {
        let mut solver = tautosolver::TautoSolver::new(cnf.var_num, cnf.clauses);
        let (is_sat, time_spent) = solver.solve();
        println!("Solved and obtained : {}", if is_sat {"\x1b[32mSAT\x1b[0m"}  else {"\x1b[31mUNSAT\x1b[0m"});
        if verbose {
            println!("Solved in {} seconds", time_spent.as_secs_f64());
        }
    }
}

fn help() {
    println!("This function imlements different SAT solvers.");
    println!("To use it, you may write `sat_solver [args] file(s)`");
    println!("Default is an optimization that choose which one to use.");
    println!();
    println!("-h --help    Show this message");
    println!("--cdcl       Using the CDCL solver");
    println!("--khorn      Using the Khorn solver");
    println!("--dummy      Use the dummy solver");
    println!("-v --verbose Print the model and different informations");
    println!("-t --time    Limit the maximum time to spend searching");
    println!("For example `./sat_solver --cdcl tests/sat/horn1.cnf tests/unsat/php6-4.cnf` will returns :");
    println!("SAT\nUNSAT");
}


fn main() {
    let args: Vec<String> = std::env::args().collect();

    let (flags, files) = get_args(args);

    if flags.contains(&"-h".to_owned()) || flags.contains(&"--help".to_owned()) {
        help();
        std::process::exit(0);
    }

    let cnfs = get_cnfs(files);
    let max_time = {
        let time = flags.iter().find(|&f| f.parse::<u64>().is_ok());
        if time.is_none() {
            None
        } else {
            Some(std::time::Duration::from_secs(time.unwrap().parse::<u64>().ok().unwrap()))
        }
    };

    let verbose = flags.contains(&"-v".to_owned()) || flags.contains(&"--verbose".to_owned());
    
    if !flags.iter().any(|f| f.starts_with('-') && !(f.starts_with("-v")) && !(f.starts_with("--v"))) {
        // No flags (other than the timer or verbose) are set
        for cnf in cnfs.clone() {
            if khorn::is_khorn(&cnf) {
                khorn_solver(vec![cnf], max_time, verbose);
            } else {
                quick_solver(vec![cnf], max_time, verbose)
            }
        }
    }
    if flags.contains(&"--cdcl".to_owned()) {
        quick_solver(cnfs.clone(), max_time, verbose);
    }
    if flags.contains(&"--khorn".to_owned()) {
        if verbose && !khorn::is_khorn(&cnfs[0]) { println!("Not a khorn configuration but go on") }
        khorn_solver(cnfs.clone(), max_time, verbose);
    }
    if flags.contains(&"--dummy".to_owned()) {
        dummy_solver(cnfs.clone(), max_time, verbose);
    }
}



#[cfg(test)]
mod tests {
    use std::process::exit;
    use super::*;

    use super::parser::*;
    use super::solver::Solver;

    use walkdir::WalkDir;
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
                            if !sat_model_check(tmp_clauses.as_slice(), &solver.models) {
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

