pub trait Solver<'a> {
    fn new<'b: 'a>(cnf: &'b crate::all_types::Cnf) -> Self;
    fn solve(&mut self) -> bool;
    fn assigns(&mut self) -> &Vec<crate::all_types::BoolValue>;
}