#[derive(Clone)]
pub struct OrPredicate {
    pub left: super::RcPredicate,
    pub right: super::RcPredicate
}

pub trait ToOrPredicate {
    fn or(&self, val: super::RcPredicate) -> super::RcPredicate;
}

impl super::Predicate for OrPredicate { }