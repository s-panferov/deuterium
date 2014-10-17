
use query::{Query, RcQuery};

#[deriving(Send, Clone)]
pub struct ExcludeQuery {
    pub query: RcQuery
}

pub trait ToExcludeQuery {
    fn exclude(&self) -> RcQuery;
}

impl Query for ExcludeQuery {}

impl ToExcludeQuery for RcQuery { 
    fn exclude(&self) -> RcQuery {
        ExcludeQuery{query: self.clone()}.upcast()
    }
}