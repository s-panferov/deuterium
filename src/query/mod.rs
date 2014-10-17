
use std::sync::Arc;

use to_sql::{ToSql, QueryToSql};

pub use self::raw::{RawQuery};
pub use self::is::{IsQuery, ToIsQuery};
pub use self::is_null::{IsNullQuery, ToIsNullQuery};
pub use self::or::{OrQuery, ToOrQuery};
pub use self::and::{AndQuery, ToAndQuery};
pub use self::exclude::{ExcludeQuery, ToExcludeQuery};
pub use self::within::{
    InQuery, ToInQuery
};

pub use self::range::{
    InRangeQuery, ToInRangeQuery,
    InRangeBounds, ExcludeBoth, IncludeBoth, ExcludeRight, ExcludeLeft
};

pub use self::inequality::{
    InequalityQuery, ToInequalityQuery,
    Inequality, LessThan, LessThanEqual, GreaterThan, GreaterTranEqual
};

mod is;
mod is_null;
mod or;
mod within;
mod range;
mod and;
mod inequality;
mod exclude;
mod raw;

pub trait Query: Sync + Send + QueryToSql { 
    fn upcast(self) -> RcQuery {
        Arc::new(box self as BoxedQuery)
    }
}

impl ToOrQuery for RcQuery {
    fn or(&self, query: RcQuery) -> RcQuery {
        OrQuery{ left: self.clone(), right: query }.upcast()
    }
}

impl ToAndQuery for RcQuery {
    fn and(&self, query: RcQuery) -> RcQuery {
        AndQuery{ left: self.clone(), right: query }.upcast()
    }
}

pub type BoxedQuery = Box<Query + Send + Sync>;
pub type RcQuery = Arc<BoxedQuery>;