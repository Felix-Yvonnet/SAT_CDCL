use rand::prelude::IteratorRandom;
use std::ops::{Index, IndexMut};

#[derive(Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
pub struct Lit(u32);
impl Lit {
    #[inline]
    pub fn get_var(self) -> Var {
        Var(self.0 >> 1)
    }
    #[inline]
    pub fn is_pos(&self) -> bool {
        self.0 & 1 == 0
    }
    #[inline]
    pub fn is_neg(&self) -> bool {
        self.0 & 1 != 0
    }
}
impl From<i32> for Lit {
    #[inline]
    fn from(x: i32) -> Self {
        debug_assert!(x != 0);
        let d: u32 = x.unsigned_abs() - 1;
        if x > 0 {
            Lit(d << 1)
        } else {
            Lit((d << 1) + 1)
        }
    }
}
impl std::ops::Not for Lit {
    type Output = Self;
    #[inline]
    fn not(self) -> Self::Output {
        Lit(self.0 ^ 1)
    }
}
impl<T> Index<Lit> for Vec<T> {
    type Output = T;
    #[inline]
    fn index(&self, lit: Lit) -> &Self::Output {
        &self[lit.0 as usize]
    }
}
impl<T> IndexMut<Lit> for Vec<T> {
    #[inline]
    fn index_mut(&mut self, lit: Lit) -> &mut Self::Output {
        &mut self[lit.0 as usize]
    }
}

#[derive(Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
pub struct Var(pub u32);
impl Var {
    pub fn from_id(x: usize) -> Var {
        Var(x as u32)
    }
}

impl<T> Index<Var> for Vec<T> {
    type Output = T;
    #[inline]
    fn index(&self, var: Var) -> &Self::Output {
        &self[var.0 as usize]
    }
}
impl<T> IndexMut<Var> for Vec<T> {
    #[inline]
    fn index_mut(&mut self, var: Var) -> &mut Self::Output {
        &mut self[var.0 as usize]
    }
}

pub type Clause = Vec<Lit>;

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct CClause {
    clause: Vec<Lit>,
    pub pos: Option<Var>,
    pub len: usize,
    pub is_present: bool,
}

impl CClause {
    pub fn new(clause: Vec<Lit>, pos: Option<Var>) -> Self {
        let n = clause.len();
        CClause {
            clause,
            pos,
            len: n,
            is_present: false,
        }
    }
    pub fn iter(&self) -> impl Iterator<Item = &Lit> {
        self.clause.iter()
    }
    pub fn len(&self) -> usize {
        self.len
    }
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }
    pub fn get_first(&self) -> Lit {
        self.clause[0]
    }
    pub fn get_at_pos(&self, pos: usize) -> Lit {
        self.clause[pos]
    }
    pub fn decr_len(&mut self) {
        self.len -= 1
    }
}

#[derive(Debug, Default, Clone)]

pub struct CAllClauses {
    pub clauses: Vec<CClause>,
}
impl CAllClauses {
    pub fn new(clauses: Vec<CClause>) -> Self {
        CAllClauses { clauses }
    }
    pub fn iter(&mut self) -> impl Iterator<Item = &mut CClause> {
        self.clauses.iter_mut()
    }
}

#[derive(Debug, Default, Clone)]

pub struct AllClauses {
    pub clauses: Vec<Clause>,
}

impl AllClauses {
    pub fn push(&mut self, clause: Clause) {
        self.clauses.push(clause);
    }
}

#[derive(Debug, Clone)]
pub struct CNF {
    pub var_num: usize,
    pub cl_num: usize,
    pub clauses: Vec<Vec<Lit>>,
}

impl CNF {
    pub fn iter(&mut self) -> impl Iterator<Item = &Vec<Lit>> {
        self.clauses.iter()
    }
}

#[derive(PartialEq, Debug, Copy, Clone)]
#[repr(u8)]
pub enum BoolValue {
    True = 0,
    False = 1,
    Undefined = 2,
}

impl From<i8> for BoolValue {
    #[inline]
    fn from(x: i8) -> Self {
        match x {
            0 => BoolValue::True,
            1 => BoolValue::False,
            _ => BoolValue::Undefined,
        }
    }
}
impl std::ops::Not for BoolValue {
    type Output = Self;
    #[inline]
    fn not(self) -> Self::Output {
        match self {
            Self::True => Self::False,
            Self::False => Self::True,
            Self::Undefined => Self::Undefined,
        }
    }
}

#[derive(Debug, Default)]
pub struct ImplGraph(Vec<Vec<Lit>>);
// If p in impl_graph[q] then p goes to q in the implication graph

#[derive(Debug, Default)]
pub struct WorkingModel {
    // The working assignment of the model
    assigns: Vec<BoolValue>,
    // The decision level of each var
    decision_level: Vec<(usize, usize)>,
    // The implication graph
    impl_graph: ImplGraph,
}

impl WorkingModel {
    pub fn new(n: usize) -> WorkingModel {
        WorkingModel {
            assigns: vec![BoolValue::Undefined; n],
            decision_level: vec![(0, 0); n],
            impl_graph: ImplGraph(vec![Vec::new(); n]),
        }
    }
    pub fn assign(&mut self, var: Var, value: BoolValue, level: usize, number: usize) {
        self.assigns[var] = value;
        self.decision_level[var] = (level, number);
    }
    #[inline]
    pub fn level(&self, v: Var) -> usize {
        self.decision_level[v].0
    }
    #[inline]
    pub fn precise_level(&self, v: Var) -> (usize, usize) {
        self.decision_level[v]
    }
    #[inline]
    pub fn eval(&self, lit: Lit) -> BoolValue {
        BoolValue::from(self.assigns[lit.get_var()] as i8 ^ lit.is_neg() as i8)
    }

    pub fn impl_is_empty(&self, var: Var) -> bool {
        self.impl_graph.0[var].is_empty()
    }
    // adds implication in implication graph
    // by taking unit clause and unnassigned variable that is to be set true in argument
    pub fn add_implications(&mut self, var: Var, clause: Clause) {
        for lit in clause.iter() {
            if lit.get_var() != var {
                self.impl_graph.0[var].push(!*lit)
            }
        }
    }
    pub fn find_conflict(&self, conflict: &Clause) -> Clause {
        // backtracking the implication graph to find the sources of the conflict
        // creates the conflict clause
        let mut stack = Vec::new();
        let mut conflict_clause = Vec::new();
        for lit in conflict {
            stack.push(!*lit)
        }
        while !stack.is_empty() {
            let lit = stack.pop().unwrap();
            if self.impl_graph.0[lit.get_var()].is_empty() && !conflict_clause.contains(&!lit) {
                conflict_clause.push(!lit)
            } else {
                for lit_dep in &self.impl_graph.0[lit.get_var()] {
                    stack.push(*lit_dep)
                }
            }
        }
        conflict_clause
    }

    // evaluate the state of each clause
    pub fn state_clause(&self, clause: &Clause) -> BoolValue {
        let mut state_clause = BoolValue::False;
        for lit in clause {
            match self.eval(*lit) {
                BoolValue::True => {
                    return BoolValue::True;
                }
                BoolValue::Undefined => {
                    state_clause = BoolValue::Undefined;
                }
                _ => {}
            }
        }
        return state_clause;
    }

    // evaluate the state of the formula
    pub fn state_formula(&self, formula: &AllClauses) -> BoolValue {
        let mut is_undefined = false;
        for clause in &formula.clauses {
            match self.state_clause(clause) {
            BoolValue::False => return BoolValue::False,
            BoolValue::Undefined => is_undefined = true,
            _ => {}
            } 
        }
        if is_undefined {
            BoolValue::Undefined
        } else {
            BoolValue::True
        }
    }

    // find conflict when state of the formula is false
    pub fn conflicting(&self, formula: &AllClauses) -> Option<Clause> {
        for clause in &formula.clauses {
            if self.state_clause(clause) == BoolValue::False {
                return Some(clause.to_vec());
            }
        }
        None
    }

    // checks whether a clause is a unit clause
    // ie all its literals are false except one which is undefined
    pub fn is_unit_clause(&self, clause: &Clause) -> Option<Lit> {
        let mut undefined_lit = None;
        for lit in clause {
            match self.eval(*lit) {
                BoolValue::True => return None,
                BoolValue::Undefined => {
                    if undefined_lit.is_some() {
                        return None;
                    } else {
                        undefined_lit = Some(*lit)
                    }
                }
                _ => {}
            }
        }
        undefined_lit
    }

    pub fn all_good(&self, clauses: &AllClauses) -> bool {
        for clause in clauses.clauses.iter() {
            let mut is_verified = false;
            for lit in clause.iter() {
                if self.eval(*lit) == BoolValue::True {
                    is_verified = true;
                    break;
                }
            }
            if !is_verified {
                return false;
            }
        }
        true
    }
    pub fn next_unassigned(&self) -> Var {
        for i in 0..self.assigns.len() {
            if self.assigns[i] == BoolValue::Undefined {
                return Var::from_id(i);
            }
        }
        panic!("no variable ?")
    }
    pub fn random_unassigned(&self) -> Var {
        Var::from_id(
            (0..self.assigns.len())
                .filter(|&var| self.assigns[var] == BoolValue::Undefined)
                .choose(&mut rand::thread_rng())
                .unwrap(),
        )
    }
    #[cfg(test)]
    pub fn get_assigned(&self) -> &Vec<BoolValue> {
        &self.assigns
    }
    //implements backtracking : modifies the working model
    pub fn backtracking(&mut self, level: usize) {
        for ind in 0..self.assigns.len() {
            if self.decision_level[ind].0 > level {
                self.decision_level[ind] = (0, 0);
                self.assigns[ind] = BoolValue::Undefined;
                self.impl_graph.0[ind] = Vec::new();
            }
        }
    }
}
