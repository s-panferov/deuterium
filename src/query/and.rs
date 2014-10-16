
use query::{Query, RcQuery};

#[deriving(Send, Clone)]
pub struct AndQuery {
    pub left: RcQuery,
    pub right: RcQuery
}

pub trait ToAndQuery {
    fn and(&self, val: RcQuery) -> AndQuery;
}

impl Query for AndQuery { }