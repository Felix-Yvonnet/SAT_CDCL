use crate::*;

/// The simpliest solver one can think of.
/// It is exponential and not very efficient but useful for controlling the performance and predictions.
pub struct TautoSolver {
    n: usize,
    clauses: Vec<Clause>,
    pub assigns: Vec<BoolValue>,
}

impl TautoSolver {
    pub fn new(cnf: Cnf) -> TautoSolver {
        TautoSolver {
            n: cnf.var_num,
            clauses: cnf.clauses,
            assigns: vec![BoolValue::Undefined; cnf.var_num],
        }
    }

    #[allow(dead_code)]
    pub fn assigns(&self) -> &Vec<BoolValue> {
        &self.assigns
    }

    pub fn solve(&mut self) -> bool {
        self.ssolve(0)
    }

    fn eval(&self) -> bool {
        for clause in self.clauses.iter() {
            let mut satisfied = false;
            for lit in clause {
                match self.assigns[lit.get_var()] {
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

    fn ssolve(&mut self, i: usize) -> bool {
        if i == self.n {
            return self.eval();
        }
        self.assigns[i] = BoolValue::True;
        let result = self.ssolve(i + 1);

        if !result {
            self.assigns[i] = BoolValue::False;
            return self.ssolve(i + 1);
        }
        true
    }
}
