
use query::{Query, RcQuery};

#[deriving(Send, Clone)]
pub struct OrQuery {
    pub left: RcQuery,
    pub right: RcQuery
}

pub trait ToOrQuery {
    fn or(&self, val: RcQuery) -> RcQuery;
}

impl Query for OrQuery { }