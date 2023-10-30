use crate::*;

/// The DPLL algorithm is a bit better than the naive one.
/// To show the importance of clause learning in CDCL algorithm
pub struct Dpll {
    n: usize,
    clauses: Vec<Clause>,
    pub assigns: Vec<BoolValue>,
}

impl<'a> solver::Solver<'a> for Dpll {
    fn new<'b>(cnf: &'b Cnf) -> Dpll
    where
        'b: 'a,
    {
        Dpll {
            n: cnf.var_num,
            clauses: cnf.clauses.clone(),
            assigns: vec![],
        }
    }

    fn assigns(&mut self) -> &Vec<BoolValue> {
        &self.assigns
    }

    fn solve(&mut self) -> bool {
        let vars = &mut vec![false; self.n];
        if Dpll::dpll(&self.clauses, vars) {
            for val in vars.iter() {
                self.assigns.push(BoolValue::from(!val as i8));
            }
            true
        } else {
            false
        }

    }
}

impl Dpll {

    fn dpll(formula: &Vec<Clause>, mut vars: &mut Vec<bool>) -> bool {
        let mut formula = formula.to_vec();
    
        Dpll::unit_propagate(&mut formula, &mut vars);
    
        if formula.is_empty() {
            return true;
        }
    
        if formula.iter().any(|clause| clause.is_empty()) {
            return false;
        }
    
        let var = Dpll::find_dominant_variable(&formula, vars.len());
    
        formula.push(vec![Lit::from(var as i32)]);
        if Dpll::dpll(&formula, &mut vars) {
            return true;
        }

    
        formula.pop();
        formula.push(vec![Lit::from(-(var as i32))]);

        Dpll::dpll(&formula, &mut vars)
    }
    
    fn unit_propagate(mut formula: &mut Vec<Clause>, vars: &mut Vec<bool>,) {
        while let Some(clause) = formula.iter().find(|clause| clause.len() == 1) {
            let lit = clause[0];
            let var = lit.get_var();
            let truth = lit.is_pos();
            vars[var] = truth;
            Dpll::ramove_all_useless(&mut formula, lit);
        }
    }

    fn ramove_all_useless(formula: &mut Vec<Clause>, lit: Lit) {
    
        let mut index: usize = 0;
        while index < formula.len() {
            let clause = &mut formula[index];
    
            if clause.contains(&lit) {
                let n = formula.len();
                formula.swap(index, n - 1);
                formula.pop();
                continue;
            }
    
            let mut lit_num = 0;
            while lit_num < clause.len() {
                if clause[lit_num] == !lit {
                    let n = clause.len();
                    clause.swap(lit_num, n - 1);
                    clause.pop();
                    continue;
                }
                lit_num += 1;
            }
    
            index += 1;
        }
    }
    
    fn find_dominant_variable(formula: &Vec<Clause>, n_vars: usize) -> usize {
        let mut freqs = vec![0; n_vars];
    
        for clause in formula {
            for lit in clause {
                let i = lit.get_var();
                freqs[i] += 1;
            }
        }
    
        let mut max: i32 = 0;
        let mut argmax: usize = 0;
    
        for (i, &freq) in freqs.iter().enumerate() {
            if freq > max {
                max = freq;
                argmax = i;
            }
        }
    
        return argmax+1;
    }

}
