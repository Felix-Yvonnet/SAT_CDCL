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
            assigns: vec![BoolValue::Undefined; cnf.var_num],
        }
    }

    fn assigns(&mut self) -> &Vec<BoolValue> {
        &self.assigns
    }

    fn solve(&mut self) -> bool {
        let clauses = self.clauses.clone();
        self.dpll(&clauses)
    }
}

impl Dpll {
    fn dpll(&mut self, formula: &[Clause]) -> bool {
        let mut formula = formula.to_owned();

        self.unit_propagation(&mut formula);

        if formula.is_empty() {
            return true;
        }

        if formula.iter().any(|clause| clause.is_empty()) {
            return false;
        }

        let var = self.next_unassigned(&formula);

        formula.push(vec![Lit::from(var as i32 + 1)]);
        if self.dpll(&formula) {
            return true;
        }

        formula.pop();
        formula.push(vec![!Lit::from(var as i32 + 1)]);
        self.dpll(&formula)
    }

    fn unit_propagation(&mut self, formula: &mut Vec<Clause>) {
        while let Some(clause) = formula.iter().find(|clause| clause.len() == 1) {
            let lit = clause[0];
            self.assigns[lit.get_var()] = BoolValue::from(lit.is_neg() as i8);
            self.remove_useless(formula, lit);
        }
    }

    fn remove_useless(&self, formula: &mut Vec<Clause>, lit: Lit) {
        let mut index = 0;
        while index < formula.len() {
            let clause = &mut formula[index];
            if clause.contains(&lit) {
                let len = formula.len();
                formula.swap(index, len - 1);
                formula.pop();
                continue;
            }

            let mut neg_index = 0;
            while neg_index < clause.len() {
                if clause[neg_index] == !lit {
                    let len = clause.len();
                    clause.swap(neg_index, len - 1);
                    clause.pop();
                    continue;
                }
                neg_index += 1;
            }

            index += 1;
        }
    }

    fn next_unassigned(&self, formula: &Vec<Clause>) -> usize {
        let mut frequences = vec![0; self.n];
        for clause in formula {
            for lit in clause {
                frequences[lit.get_var()] += 1;
            }
        }

        let mut max = 0;
        let mut argmax = 0;

        for (i, &freq) in frequences.iter().enumerate() {
            if freq > max {
                max = freq;
                argmax = i;
            }
        }
        argmax
    }
}
