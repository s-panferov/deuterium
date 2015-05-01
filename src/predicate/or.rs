use super::{ToSharedPredicate};

#[derive(Clone)]
pub struct OrPredicate {
    pub left: super::SharedPredicate,
    pub right: super::SharedPredicate
}

pub trait ToOrPredicate {
    fn or(&self, val: super::SharedPredicate) -> super::SharedPredicate;
}

impl super::Predicate for OrPredicate { }

impl ToOrPredicate for super::SharedPredicate {
    fn or(&self, predicate: super::SharedPredicate) -> super::SharedPredicate {
        OrPredicate{ left: self.clone(), right: predicate }.upcast()
    }
}
