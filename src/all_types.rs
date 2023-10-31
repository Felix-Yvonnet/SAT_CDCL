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

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CClause<'a> {
    clause: &'a Vec<Lit>,
    pub pos: Option<Var>,
}

impl<'a> CClause<'a> {
    pub fn new<'b: 'a>(clause: &'b Vec<Lit>, pos: Option<Var>) -> Self {
        CClause { clause, pos }
    }
    pub fn iter(&self) -> impl Iterator<Item = &Lit> {
        self.clause.iter()
    }
}

#[derive(Debug, Default, Clone)]

pub struct CAllClauses<'a> {
    pub clauses: Vec<CClause<'a>>,
}
impl<'a> CAllClauses<'a> {
    pub fn new(clauses: Vec<CClause<'a>>) -> Self {
        CAllClauses { clauses }
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
    pub fn len(&self) -> usize {
        self.clauses.len()
    }
    #[allow(dead_code)]
    pub fn is_empty(&self) -> bool {
        self.clauses.is_empty()
    }
}

#[derive(Debug, Clone)]
pub struct Cnf {
    pub var_num: usize,
    pub cl_num: usize,
    pub clauses: Vec<Vec<Lit>>,
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
/// If p in impl_graph[q] then p goes to q in the implication graph
pub struct ImplGraph(Vec<Vec<Lit>>);

#[derive(Debug)]
pub struct WorkingModel {
    // The working assignment of the model
    assigns: Vec<BoolValue>,
    // The decision level of each var
    decision_level: Vec<usize>,
    // The implication graph
    impl_graph: ImplGraph,
    // VSIDS
    pub heap: Heap,
}

impl WorkingModel {
    pub fn new(n: usize) -> WorkingModel {
        WorkingModel {
            assigns: vec![BoolValue::Undefined; n],
            decision_level: vec![0; n],
            impl_graph: ImplGraph(vec![Vec::new(); n]),
            heap: Heap::new(n, 1.0),
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
        BoolValue::from(self.assigns[lit.get_var()] as i8 ^ lit.is_neg() as i8)
    }
    #[inline]
    pub fn state_var(&self, var: Var) -> BoolValue {
        self.assigns[var]
    }

    /// adds implication in implication graph
    /// by taking unit clause and unnassigned variable that is to be set true in argument
    pub fn add_implications(&mut self, var: Var, clause: &Clause) {
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
        while let Some(lit) = stack.pop() {
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

    /// evaluate the state of each clause
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
        state_clause
    }

    /// evaluate the state of the formula
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

    /// find conflict when state of the formula is false
    pub fn conflicting(&self, formula: &AllClauses) -> Option<Clause> {
        for clause in &formula.clauses {
            if self.state_clause(clause) == BoolValue::False {
                return Some(clause.to_vec());
            }
        }
        None
    }

    /// Checks whether a clause is a unit clause
    /// ie all its literals are false except one which is undefined
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

    #[allow(dead_code)]
    pub fn next_unassigned(&self) -> Option<Var> {
        for i in 0..self.assigns.len() {
            if self.assigns[i] == BoolValue::Undefined {
                return Some(Var::from_id(i));
            }
        }
        None
    }
    #[allow(dead_code)]
    pub fn random_unassigned(&self) -> Var {
        Var::from_id(
            (0..self.assigns.len())
                .filter(|&var| self.assigns[var] == BoolValue::Undefined)
                .choose(&mut rand::thread_rng())
                .unwrap(),
        )
    }
    pub fn get_assigned(&self) -> &Vec<BoolValue> {
        &self.assigns
    }
    //implements backtracking : modifies the working model
    pub fn backtracking(&mut self, level: usize) {
        for ind in 0..self.assigns.len() {
            if self.decision_level[ind] > level {
                if !self.heap.contains(Var::from_id(ind)) {
                    self.heap.push(Var::from_id(ind));
                }
                self.decision_level[ind] = 0;
                self.assigns[ind] = BoolValue::Undefined;
                self.impl_graph.0[ind] = Vec::new();
            }
        }
    }
}

#[derive(Debug)]
/// Implements 2 watched literals.
/// Try to keep as invariant the fact that if a literal is false among them then we should unit propagate the other.
/// We also want to have a literal if it is true so that we can tell the evaluation of the clause just by looking at its literals.
pub struct Watcher {
    // watcher[lit] = list of clauses where "lit" is watched
    // usize such that solver.clauses[usize] is the clauses we are interested in
    lit_to_clauses: Vec<Vec<usize>>,
}

impl Watcher {
    pub fn new(var_num: usize) -> Self {
        Watcher {
            lit_to_clauses: vec![vec![]; var_num * 2],
        }
    }
    pub fn add(&mut self, lit: Lit, clause: usize) {
        self.lit_to_clauses[lit].push(clause);
    }
}

#[derive(Debug, Clone)]
pub struct Heap {
    heap: Vec<Var>,
    indices: Vec<Option<usize>>,
    activity: Vec<f64>,
    bump_inc: f64,
}
impl Heap {
    pub fn new(n: usize, bump_inc: f64) -> Heap {
        Heap {
            heap: (0..n).map(|x| Var(x as u32)).collect(),
            indices: (0..n).map(Some).collect(),
            activity: vec![0.0; n],
            bump_inc,
        }
    }

    fn gt(&self, left: Var, right: Var) -> bool {
        self.activity[left] > self.activity[right]
    }
    pub fn decay_inc(&mut self) {
        self.bump_inc *= 1.05;
    }
    pub fn incr_activity(&mut self, v: Var) {
        self.activity[v] += self.bump_inc;

        if self.activity[v] >= 1e100 {
            self.activity.iter_mut().for_each(|x| *x *= 1e-100);
            self.bump_inc *= 1e-100;
        }
        if self.contains(v) {
            let index = self.indices[v].unwrap();
            self.increase(index);
        }
    }

    pub fn pop(&mut self) -> Option<Var> {
        if self.heap.is_empty() {
            return None;
        }
        let x = self.heap[0];
        self.indices[x] = None;
        if self.heap.len() > 1 {
            self.heap[0] = *self.heap.last().unwrap();
            self.indices[self.heap[0]] = Some(0);
        }
        self.heap.pop();
        if self.heap.len() > 1 {
            self.decrease(0);
        }
        Some(x)
    }

    fn increase(&mut self, i: usize) {
        if i == 0 {
            return;
        }
        let mut index = i;
        let x = self.heap[index];
        let mut par = (index - 1) >> 1;
        loop {
            if !self.gt(x, self.heap[par]) {
                break;
            }
            self.heap[index] = self.heap[par];
            self.indices[self.heap[par]] = Some(index);
            index = par;
            if index == 0 {
                break;
            }
            par = (par - 1) >> 1;
        }
        self.heap[index] = x;
        self.indices[x] = Some(index);
    }

    fn decrease(&mut self, i: usize) {
        let x = self.heap[i];
        let mut index = i;
        while 2 * index + 1 < self.heap.len() {
            let left = 2 * index + 1;
            let right = left + 1;
            let child = if right < self.heap.len() && self.gt(self.heap[right], self.heap[left]) {
                right
            } else {
                left
            };
            if self.gt(self.heap[child], x) {
                self.heap[index] = self.heap[child];
                self.indices[self.heap[index]] = Some(index);
                index = child;
            } else {
                break;
            }
        }
        self.heap[index] = x;
        self.indices[x] = Some(index);
    }

    fn push(&mut self, v: Var) {
        if self.contains(v) {
            return;
        }
        while (v.0 as usize) >= self.indices.len() {
            self.indices.push(None);
            self.activity.push(0.0);
        }
        self.indices[v] = Some(self.heap.len());
        self.heap.push(v);
        self.increase(self.indices[v].expect("No index"));
    }

    fn contains(&mut self, v: Var) -> bool {
        (v.0 as usize) < self.indices.len() && self.indices[v].is_some()
    }
}
