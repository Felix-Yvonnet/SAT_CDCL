use crate::*;

/// The simpliest solver one can think of.
/// It is exponential and not very efficient but useful for controlling the performance and predictions.
pub struct TautoSolver {
    n: usize,
    clauses: Vec<Clause>,
    pub assigns: Vec<BoolValue>,
}

impl Solver for TautoSolver {
    fn new(cnf: &mut Cnf) -> TautoSolver {
        TautoSolver {
            n: cnf.var_num,
            clauses: cnf.clauses,
            assigns: vec![BoolValue::Undefined; cnf.var_num],
        }
    }
    fn assigns(&self) -> &Vec<all_types::BoolValue> {
        &self.assigns
    }
    fn specific_solve(&mut self, max_time : Option<std::time::Duration>) -> (Option<bool>, std::time::Duration) {
        let start = std::time::Instant::now();
        (self.ssolve(0, start, max_time), start.elapsed())
    }
}

impl TautoSolver {

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

    fn ssolve(
        &mut self,
        i: usize,
        start: std::time::Instant,
        max_time: Option<std::time::Duration>,
    ) -> Option<bool> {
        if i == self.n {
            return Some(self.eval());
        }
        self.assigns[i] = BoolValue::True;
        let result = self.ssolve(i + 1, start, max_time);

        result?;

        if let Some(time) = max_time {
            if start.elapsed() > time {
                return None;
            }
        }
        if !result.unwrap() {
            self.assigns[i] = BoolValue::False;
            return self.ssolve(i + 1, start, max_time);
        }
        Some(true)
    }
}
