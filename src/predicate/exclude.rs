
use predicate::{Predicate, RcPredicate};

#[deriving(Send, Clone)]
pub struct ExcludePredicate {
    pub predicate: RcPredicate
}

pub trait ToExcludePredicate {
    fn exclude(&self) -> RcPredicate;
}

impl Predicate for ExcludePredicate {}

impl ToExcludePredicate for RcPredicate { 
    fn exclude(&self) -> RcPredicate {
        ExcludePredicate{predicate: self.clone()}.upcast()
    }
}