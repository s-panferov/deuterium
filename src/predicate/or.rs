
use predicate::{Predicate, RcPredicate};

#[deriving(Clone)]
pub struct OrPredicate {
    pub left: RcPredicate,
    pub right: RcPredicate
}

pub trait ToOrPredicate {
    fn or(&self, val: RcPredicate) -> RcPredicate;
}

impl Predicate for OrPredicate { }