#[derive(Clone)]
pub struct OrPredicate {
    pub left: super::SharedPredicate,
    pub right: super::SharedPredicate
}

pub trait ToOrPredicate {
    fn or(&self, val: super::SharedPredicate) -> super::SharedPredicate;
}

impl super::Predicate for OrPredicate { }