mod all_types;
mod khorn;
mod parser;
mod sat2;
mod solver;
mod tautosolver;
use core::panic;

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

fn get_cnfs(files: Vec<String>, verbose: bool) -> Vec<Cnf> {
    let mut cnfs: Vec<Cnf> = Vec::new();
    for file in files {
        let cnf: crate::Cnf = parser::parse_cnf(&file, verbose).unwrap();
        cnfs.push(cnf);
    }
    cnfs
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
    println!("--2sat        Using the 2sat solver");
    println!("--proof       Test whether the returned assigments are correct (the ouput model indeed satisfies the problem)");
    println!("-v --verbose  Print the model and different informations");
    println!("For example `./sat_solver --cdcl tests/sat/horn1.cnf tests/unsat/php6-4.cnf` will returns :");
    println!("SAT\nUNSAT");
}

fn print_status(is_sat: bool) {
    if is_sat {
        println!("s \x1b[32mSATISFIABLE\x1b[0m")
    } else {
        println!("s \x1b[31mUNSATISFIABLE\x1b[0m")
    }
}
fn print_proof(proof: bool, assigns: &[BoolValue], formula: &[Clause], verbose: bool) {
    if proof {
        sat_model_check(formula, assigns);
        print!("v ");
        for (var, eval) in assigns.iter().enumerate() {
            if *eval == BoolValue::False {
                print!("-")
            }
            print!("{} ", var + 1)
        }
        println!();

        if verbose {
            if !sat_model_check(formula, assigns) {
                println!("\x1b[31mERROR!\x1b[0m the given model does not satisfy the formula. There must be a mistake in the code.")
            } else {
                println!("This model satisfies the formula as expected.")
            }
        }
    }
}


fn sat_model_check(clauses: &[Clause], assigns: &[BoolValue]) -> bool {
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



fn apply_solver<'a>(solver: &mut impl Solver<'a>, cnf: &Cnf, verbose: bool, proof: bool) {

    let start = std::time::Instant::now();
    let is_sat = solver.solve();
    print_status(is_sat);
    print_proof(proof, solver.assigns(), &cnf.clauses, verbose);
    if verbose {
        println!("Solved in {} seconds", start.elapsed().as_secs_f64())
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let (flags, files) = get_args(args);

    let mut verbose = false;
    let mut proof = false;
    for flag in flags.iter() {
        if flag == "-v" || flag == "--verbose" {
            verbose = true;
        } else if flag == "--proof" {
            proof = true;
        } else if flag == "-h" || flag == "--help" {
            help();
            std::process::exit(0);
        }
    }

    if files.is_empty() {
        eprintln!("Please provide at least one file to resolve");
        std::process::exit(5)
    }

    let mut cnfs = get_cnfs(files, verbose);
    let mut solver_type = vec![];
    for flag in flags {
        if flag == "--cdcl" {
            solver_type.push("cdcl");
        } else if flag == "--khorn" {
            if verbose && !khorn::is_khorn(&cnfs[0]) {
                println!("\x1b[31mNot a Horn\x1b[0m configuration but go on")
            }
            solver_type.push("khorn");

        } else if flag == "--dummy" {
            solver_type.push("dummy");
        } else if flag == "--2sat" {
            solver_type.push("2sat");
        }
    }

    for cnf in cnfs.iter_mut() {
        if solver_type.is_empty() {
            if sat2::is_2sat(&cnf) {
                let mut solver = sat2::SAT2::new(cnf);
                apply_solver(&mut solver, cnf, verbose, proof)
            } else if khorn::is_khorn(&cnf) {
                let mut solver = khorn::KhornSolver::new(cnf);
                apply_solver(&mut solver, cnf, verbose, proof)
            } else {
                let mut solver = solver::CdclSolver::new(cnf);
                apply_solver(&mut solver, cnf, verbose, proof)
            }
        } else {
            for &solver_name in solver_type.iter() {
                match solver_name {
                    "cdcl" => {
                    let mut solver = solver::CdclSolver::new(cnf);
                    apply_solver(&mut solver, cnf, verbose, proof)
                    }
                    "2sat" => {
                        let mut solver = sat2::SAT2::new(cnf);
                        apply_solver(&mut solver, cnf, verbose, proof)
                    }
                    "khorn" => {
                        let mut solver = khorn::KhornSolver::new(cnf);
                        apply_solver(&mut solver, cnf, verbose, proof)
                    }
                    "dummy" => {
                        let mut solver = tautosolver::TautoSolver::new(cnf);
                        apply_solver(&mut solver, cnf, verbose, proof)
                    }
                    _ => panic!("Weird name")
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::process::exit;
    // use crate::tautosolver::TautoSolver;

    use super::*;

    use super::parser::*;
    use super::solver::CdclSolver;

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
        let entries = WalkDir::new(format!("tests/small/{which}/"));
        for entry in entries
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| !e.file_type().is_dir())
        {
            let path_str = entry.path().to_str().unwrap();

            if path_str.ends_with(".cnf") {
                let mut cnf = parse_cnf(path_str, false).unwrap();
                let tmp_clauses = cnf.clauses.clone();
                let mut solver = CdclSolver::new(&mut cnf);
                let status = solver.solve();

                if status == expected {
                    if status && !sat_model_check(tmp_clauses.as_slice(), solver.assigns()) {
                        self::panic!(
                            "Failed in my code T_T cnf: {}, Result: {}{:?}\x1b[0m Expected: {}{:?}\x1b[0m",
                            path_str, if status {"\x1b[32m"} else {"\x1b[31m"}, status, if expected {"\x1b[32m"} else {"\x1b[31m"}, expected
                        );
                    } else {
                        eprintln!("\x1b[32mSuccess\x1b[0m")
                    }
                } else {
                    self::panic!(
                        "Mismatch cnf: {}, Result: \x1b[31m{:?}\x1b[0m Expected: \x1b[32m{:?}\x1b[0m",
                        path_str, status, expected
                    );
                }
            }
        }
    }

    #[test]
    fn test_sat() {
        test_all_files("sat");
    }
    #[test]
    fn test_unsat() {
        test_all_files("unsat");
    }
    #[test]
    #[should_panic]
    fn test_parsing() {
        parse_cnf("./tests/parsing/no_p.cnf", false).unwrap();
    }
}
