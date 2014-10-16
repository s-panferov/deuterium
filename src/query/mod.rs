
use std::sync::Arc;

use to_sql::{ToSql};

pub use self::is::{IsQuery, ToIsQuery};

mod is;

pub trait Query: Sync + Send + ToSql { 
    fn upcast(self) -> RcQuery {
        Arc::new(box self as BoxedQuery)
    }
}

pub type BoxedQuery = Box<Query + Send + Sync>;
pub type RcQuery = Arc<BoxedQuery>;