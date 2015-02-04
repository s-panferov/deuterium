
use predicate::{Predicate, RcPredicate};

#[derive(Clone)]
pub struct AndPredicate {
    pub left: RcPredicate,
    pub right: RcPredicate
}

pub trait ToAndPredicate {
    fn and(&self, val: RcPredicate) -> RcPredicate;
}

impl Predicate for AndPredicate { }