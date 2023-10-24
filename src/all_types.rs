use std::ops::{Index, IndexMut};
use rand::prelude::IteratorRandom;

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
        let d: u32 = x.abs() as u32 - 1;
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
        CClause { clause: clause, pos: pos, len: n, is_present: false }
    }
    pub fn iter(&self) -> impl Iterator<Item=&Lit> {
        self.clause.iter()
    }
    pub fn len(&self) -> usize {
        self.len
    }
    pub fn get_first(&self) -> Lit {
        self.clause[0]
    }
    pub fn get_at_pos(&self, pos: usize) -> Lit {
        self.clause[pos]
    }
    pub fn decr_len(&mut self) {
        self.len-=1
    }
}


#[derive(Debug, Default, Clone)]

pub struct CAllClauses {
    pub clauses: Vec<CClause>,
}
impl CAllClauses {
    pub fn new(clauses: Vec<CClause>) -> Self {
        CAllClauses { clauses: clauses }
    }
    pub fn iter(&mut self) -> impl Iterator<Item=&mut CClause> {
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
    pub fn iter(&mut self) -> impl Iterator<Item=&Vec<Lit>> {
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
pub struct WorkingModel {
    // The working assignment of the model
    assigns: Vec<BoolValue>,
    // The decision level of each var
    decision_level: Vec<(usize, usize)>,
}


impl WorkingModel {
    pub fn new(n: usize) -> WorkingModel {
        WorkingModel {
            assigns: vec![BoolValue::Undefined; n],
            decision_level: vec![(0,0); n],
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
    pub fn all_good(&self, clauses: &AllClauses) -> bool {
        for clause in clauses.clauses.iter() {
            let mut is_verified = false;
            for lit in clause.iter() {
                match self.eval(*lit) {
                    BoolValue::False => {},
                    BoolValue::True => {
                        is_verified = true;
                        break
                    },
                    BoolValue::Undefined => {
                        return false;
                    }
                }
            }
            if !is_verified {
                return false;
            }
        }
        true
    }
    pub fn next_unassigned(&self) -> Var {
        Var::from_id((0..self.assigns.len()).filter(|&var| self.assigns[var] == BoolValue::Undefined).choose(&mut rand::thread_rng()).unwrap())
    }
    pub fn get_assigned(&self) -> &Vec<BoolValue> {
        &self.assigns
    }
    pub fn backtracking(&mut self, level: usize) {
        for ind in 0..self.assigns.len() {
            if self.decision_level[ind].0 > level {
                self.decision_level[ind] = (0,0);
                self.assigns[ind] = BoolValue::Undefined;
            }
        }
    }
}
