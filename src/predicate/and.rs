use super::{ToSharedPredicate};

#[derive(Clone)]
pub struct AndPredicate {
    pub left: super::SharedPredicate,
    pub right: super::SharedPredicate
}

pub trait ToAndPredicate {
    fn and(&self, val: super::SharedPredicate) -> super::SharedPredicate;
}

impl ToAndPredicate for super::SharedPredicate {
    fn and(&self, predicate: super::SharedPredicate) -> super::SharedPredicate {
        AndPredicate{ left: self.clone(), right: predicate }.upcast()
    }
}

impl super::Predicate for AndPredicate { }
