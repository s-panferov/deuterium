
use query::{Query, RcQuery};

#[deriving(Send, Clone)]
pub struct AndQuery {
    pub left: RcQuery,
    pub right: RcQuery
}

pub trait ToAndQuery {
    fn and(&self, val: RcQuery) -> RcQuery;
}

impl Query for AndQuery { }