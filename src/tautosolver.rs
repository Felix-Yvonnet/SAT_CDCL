use crate::*;

pub struct TautoSolver {
    n: usize,
    clauses: Vec<Clause>,
    assigns: Vec<BoolValue>,
}

impl TautoSolver {
    pub fn new(n: usize, clauses: Vec<Clause>) -> TautoSolver {
        TautoSolver { 
            n: n,
            clauses: clauses,
            assigns: vec![BoolValue::Undefined; n],
        }
    }

    pub fn solve(&mut self) -> (bool, std::time::Duration) {        
        let start = std::time::Instant::now();
        (self.ssolve(0), start.elapsed())

    }

    fn eval(&self) -> bool {
        let mut is_sat = true;
        for clause in self.clauses.iter() {
            let mut tmp_sat = false;
            for lit in clause {
                if self.assigns[lit.get_var()] == BoolValue::True {
                    tmp_sat = true;
                    break;
                }
            }
            if !tmp_sat {
                is_sat = false;
            }
        }
        is_sat
    }

    fn ssolve(&mut self, i: usize) -> bool {
        if i == self.n {
            return self.eval()
        }
        self.assigns[i] = BoolValue::True;
        if !self.ssolve(i+1) {
            self.assigns[i] = BoolValue::False;
            return self.ssolve(i+1)
        }
        true
    }
}