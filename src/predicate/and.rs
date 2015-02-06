
#[derive(Clone)]
pub struct AndPredicate {
    pub left: super::RcPredicate,
    pub right: super::RcPredicate
}

pub trait ToAndPredicate {
    fn and(&self, val: super::RcPredicate) -> super::RcPredicate;
}

impl super::Predicate for AndPredicate { }