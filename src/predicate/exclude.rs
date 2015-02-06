use super::{ToAbstractPredicate};

#[derive(Clone)]
pub struct ExcludePredicate {
    pub predicate: super::RcPredicate
}

pub trait ToExcludePredicate {
    fn exclude(&self) -> super::RcPredicate;
}

impl super::Predicate for ExcludePredicate {}

impl ToExcludePredicate for super::RcPredicate { 
    fn exclude(&self) -> super::RcPredicate {
        ExcludePredicate{predicate: self.clone()}.upcast()
    }
}