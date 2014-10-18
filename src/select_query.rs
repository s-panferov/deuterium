
use std::sync::Arc;

use {From};
use predicate::{RcPredicate};
use to_sql::{ToSql};

#[deriving(Clone)]
pub enum Select {
    SelectOnly(Vec<String>),
    SelectAll
}

pub trait ToSelectQuery: Send + Sync + ToSql {
    fn upcast(self) -> RcSelectQuery {
        Arc::new(box self as BoxedSelectQuery)
    }
}

#[deriving(Clone)]
pub struct SelectQuery<T> {
    pub select: Select,
    pub from: From,
    pub where_: Option<RcPredicate>
}

impl<T: Clone> SelectQuery<T> {
 
    pub fn new(select: Select, from: From) -> SelectQuery<T> {
        SelectQuery {
            select: select,
            from: from,
            where_: None
        }
    }

    pub fn where_(&self, predicate: &RcPredicate) -> SelectQuery<T> {
        let mut query = self.clone();
        query.where_ = Some(predicate.clone());
        query
    }

}

impl<T: Clone> ToSelectQuery for SelectQuery<T> {
    
}

pub type BoxedSelectQuery = Box<ToSelectQuery + Send + Sync>;
pub type RcSelectQuery = Arc<BoxedSelectQuery>;
