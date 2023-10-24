use std::io::BufRead;

pub fn parse_cnf(path: &str) -> std::io::Result<crate::all_types::CNF> {
    let input = match std::fs::File::open(path) {
        Err(e) => panic!("Impossible to open file: {}", e),
        Ok(f) => f,
    };
    println!("Reading file: {}", path);
    let reader = std::io::BufReader::new(input);
    let mut var_num = 0;
    let mut cl_num = 0;
    let mut clauses = vec![];
    for line in reader.lines() {
        let line = line?;
        let line = line.trim();

        if line.starts_with('c') {
            // comments
            continue;
        }
        let values: Vec<&str> = line.split_whitespace().collect();
        if values.is_empty() {
            // empty line
            continue;
        }
        if values[0] == "p" {
            if let Some(v) = values.get(2) {
                // Get the number of variables
                var_num = v.parse::<usize>().unwrap();
            } else {
                eprintln!("Error parsing, \"p\" line should contains the number of variables.");
            };
            if let Some(v) = values.get(3) {
                // Get the number of variables
                cl_num = v.parse::<usize>().unwrap();
            } else {
                eprintln!("Error parsing, \"p\" line should contains the number of clauses.");
            };
            continue;
        }

        let values: Vec<_> = values
            .into_iter()
            .filter_map(|x| x.parse::<i32>().ok())
            .take_while(|x| *x != 0)
            .collect();

        if values.is_empty() {
            // empty clause
            continue;
        }
        let clause: Vec<crate::all_types::Lit> = values
            .iter()
            .map(|&x| crate::all_types::Lit::from(x))
            .collect();
        clauses.push(clause);
    }
    debug_assert!(cl_num == clauses.len());
    Ok(crate::all_types::CNF {
        var_num,
        cl_num,
        clauses,
    })
}
