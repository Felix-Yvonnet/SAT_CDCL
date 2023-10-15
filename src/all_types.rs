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
    #[inline]
    pub fn var(self) -> Var {
        Var(self.0 >> 1)
    }
}
impl From<i32> for Lit {
    #[inline]
    fn from(x: i32) -> Self {
        debug_assert!(x != 0);
        let d = x.abs() as u32 - 1;
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

#[derive(Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
pub struct Var(pub u32);
impl Var {
    fn from_id(x: usize) -> Var {
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

#[derive(Debug, Default, Clone)]

pub struct AllClauses {
    pub clauses: Vec<Clause>,
}

impl AllClauses {
    pub fn push(&mut self, clause: Clause) {
        self.clauses.push(clause);
    }
}


#[derive(Debug)]
pub struct CNF {
    pub var_num: usize,
    pub clauses: Vec<Vec<Lit>>,
}


impl CNF {
    pub fn iter(&mut self) -> impl Iterator<Item=&Vec<Lit>> {
        self.clauses.iter()
    }
}


#[derive(PartialEq, Debug, Copy, Clone)]
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

#[derive(Debug, Default)]
pub struct WorkingModel {
    // The working assignment of the model
    assigns: Vec<BoolValue>,
    // The decision level of each var
    decision_level: Vec<usize>,
}


impl WorkingModel {
    pub fn new(n: usize) -> WorkingModel {
        WorkingModel {
            assigns: vec![BoolValue::Undefined; n],
            decision_level: vec![0; n],
        }
    }
    pub fn assign(&mut self, var: Var, value: BoolValue, level: usize) {
        self.assigns[var] = value;
        self.decision_level[var] = level;
    }
    #[inline]
    pub fn level(&self, v: Var) -> usize {
        self.decision_level[v]
    }
    #[inline]
    pub fn eval(&self, lit: Lit) -> BoolValue {
        BoolValue::from(self.assigns[lit.var()] as i8 ^ lit.is_neg() as i8)
    }
    pub fn all_assigned(&self) -> bool {
        !self.assigns.iter().any(|&eval| eval == BoolValue::Undefined)
    }
    pub fn next_unassigned(&self) -> Var {
        let ind = self.assigns.iter().position(|&eval| eval == BoolValue::Undefined).unwrap();
        Var::from_id(ind)
    }
    pub fn backtracking(&mut self, level: usize) {
        for ind in 0..self.assigns.len() {
            if self.decision_level[ind] >= level {
                self.decision_level[ind] = 0;
                self.assigns[ind] = BoolValue::Undefined;
            }
        }
    }
}
