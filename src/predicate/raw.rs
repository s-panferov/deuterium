
use predicate::{Predicate};

#[deriving(Send, Clone)]
pub struct RawPredicate {
    pub content: String
}

impl Predicate for RawPredicate { }