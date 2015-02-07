
#[derive(Clone)]
pub struct AndPredicate {
    pub left: super::SharedPredicate,
    pub right: super::SharedPredicate
}

pub trait ToAndPredicate {
    fn and(&self, val: super::SharedPredicate) -> super::SharedPredicate;
}

impl super::Predicate for AndPredicate { }