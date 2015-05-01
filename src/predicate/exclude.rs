use super::{ToSharedPredicate};

#[derive(Clone)]
pub struct ExcludePredicate {
    pub predicate: super::SharedPredicate
}

pub trait ToExcludePredicate {
    fn exclude(&self) -> super::SharedPredicate;
}

impl super::Predicate for ExcludePredicate {}

impl ToExcludePredicate for super::SharedPredicate {
    fn exclude(&self) -> super::SharedPredicate {
        ExcludePredicate{predicate: self.clone()}.upcast()
    }
}
