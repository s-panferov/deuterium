#[derive(Clone, Debug)]
pub struct RawPredicate {
    pub content: String
}

impl RawPredicate {
    pub fn new(content: &str) -> RawPredicate {
        RawPredicate { content: content.to_string() }
    }
}

impl super::Predicate for RawPredicate { }
