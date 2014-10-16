
use std::sync::Arc;

use to_sql::{ToSql};

pub use self::is::{IsQuery, ToIsQuery};
pub use self::or::{OrQuery, ToOrQuery};
pub use self::and::{AndQuery, ToAndQuery};

mod is;
mod or;
mod and;

pub trait Query: Sync + Send + ToSql { 
    fn upcast(self) -> RcQuery {
        Arc::new(box self as BoxedQuery)
    }
}

impl ToOrQuery for RcQuery {
    fn or(&self, query: RcQuery) -> OrQuery {
        OrQuery{ left: self.clone(), right: query }
    }
}

impl ToAndQuery for RcQuery {
    fn and(&self, query: RcQuery) -> AndQuery {
        AndQuery{ left: self.clone(), right: query }
    }
}

pub type BoxedQuery = Box<Query + Send + Sync>;
pub type RcQuery = Arc<BoxedQuery>;