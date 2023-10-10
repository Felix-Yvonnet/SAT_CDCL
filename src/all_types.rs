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


#[derive(Clone, Copy, Default, Debug)]
pub struct Clause<'a> {
    pub clause: &'a [Lit],
}

#[derive(Debug)]
pub struct CNF {
    pub var_num: Option<usize>,
    pub clause_num: Option<usize>,
    pub clauses: Vec<Vec<Lit>>,
}

#[derive(Debug, Default)]
pub struct Trail {
    stack: Vec<Lit>,
    stack_limit: Vec<usize>,
    head: usize,
}
impl Trail {
    fn new() -> Trail {
        Trail {
            stack: Vec::new(),
            stack_limit: Vec::new(),
            head: 0,
        }
    }
    fn new_descion_level(&mut self) {
        self.stack_limit.push(self.stack.len());
    }
    #[inline]
    pub fn decision_level(&self) -> usize {
        self.stack_limit.len()
    }
    #[inline]
    fn head(&self) -> Lit {
        self.stack[self.head]
    }
    #[inline]
    fn advance(&mut self) {
        self.head += 1;
    }
    fn push(&mut self, x: Lit) {
        self.stack.push(x);
    }
}


#[derive(Debug, Default)]
pub struct AssignData {
     pub assigns: Vec<Option<bool>>,
     pub decision_level: Vec<usize>,
     pub trail: Trail,
}

impl AssignData {
    fn new(n: usize) -> AssignData {
        AssignData {
            assigns: vec![None; n],
            decision_level: vec![0; n],
            trail: Trail::new(),
        }
    }
    pub fn assign(&mut self, var: Var, lb: bool, level: usize) {
        self.assigns[var] = Some(lb);
        self.decision_level[var] = level;
    }

    pub fn enqueue(&mut self, lit: Lit) {
        self.assign(
            lit.var(),
            lit.is_neg(),
            self.trail.decision_level(),
        );
        self.trail.push(lit);
    }
    
}