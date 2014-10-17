
use std::sync::Arc;

use to_sql::{ToSql};

pub use self::is::{IsQuery, ToIsQuery};
pub use self::or::{OrQuery, ToOrQuery};
pub use self::and::{AndQuery, ToAndQuery};
pub use self::within::{
    InQuery, ToInQuery,
    InRangeQuery, ToInRangeQuery,
    InRangeBounds, ExcludeBoth, IncludeBoth, ExcludeRight, ExcludeLeft
};

pub use self::inequality::{
    InequalityQuery, ToInequalityQuery,
    Inequality, LessThan, LessThanEqual, GreaterThan, GreaterTranEqual
};

mod is;
mod or;
mod within;
mod and;
mod inequality;

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