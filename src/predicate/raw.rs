
use predicate::{Predicate, RcPredicate};

#[deriving(Send, Clone)]
pub struct RawPredicate {
    pub content: String
}

impl Predicate for RawPredicate { }