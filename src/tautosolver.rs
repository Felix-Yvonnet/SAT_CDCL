use crate::*;

pub struct TautoSolver {
    n: usize,
    clauses: Vec<Clause>,
    pub assigns: Vec<BoolValue>,
}

impl TautoSolver {
    pub fn new(cnf: CNF) -> TautoSolver {
        TautoSolver {
            n: cnf.var_num,
            clauses: cnf.clauses,
            assigns: vec![BoolValue::Undefined; cnf.var_num],
        }
    }

    pub fn assigns(&self) -> Vec<BoolValue> {
        self.assigns.clone()
    }

    pub fn solve(
        &mut self,
        max_time: Option<std::time::Duration>,
    ) -> (Option<bool>, std::time::Duration) {
        let start = std::time::Instant::now();
        println!("Solving...");
        (self.ssolve(0, start, max_time), start.elapsed())
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
