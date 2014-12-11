
use predicate::{Predicate};

#[deriving(Clone)]
pub struct RawPredicate {
    pub content: String
}

impl RawPredicate {
    pub fn new(content: &str) -> RawPredicate {
        RawPredicate { content: content.to_string() }
    }
}

impl Predicate for RawPredicate { }