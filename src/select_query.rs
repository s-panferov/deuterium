
use std::sync::Arc;
use std::mem;

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
pub struct LimitOne;

#[deriving(Clone)]
pub struct LimitTwo;

#[deriving(Clone)]
pub struct LimitMany;

#[deriving(Clone)]
pub struct SelectQuery<T, L> {
    pub select: Select,
    pub from: From,
    pub where_: Option<RcPredicate>,
    pub limit: Option<uint>,
    pub offset: Option<uint>
}

impl<T: Clone, L: Clone> SelectQuery<T, L> {
 
    pub fn new(select: Select, from: From) -> SelectQuery<T, L> {
        SelectQuery {
            select: select,
            from: from,
            where_: None,
            limit: None,
            offset: None
        }
    }

    pub fn where_(&self, predicate: &RcPredicate) -> SelectQuery<T, L> {
        let mut query = self.clone();
        query.where_ = Some(predicate.clone());
        query
    }

    pub fn limit(&self, limit: uint) -> SelectQuery<T, LimitOne> {
        let mut query = self.clone();
        query.limit = Some(limit);
        unsafe{ mem::transmute(query) }
    }

    pub fn first(&self) -> SelectQuery<T, LimitOne> {
        let mut query = self.clone();
        query.limit = Some(1);
        unsafe{ mem::transmute(query) }
    }

    pub fn last(&self) -> SelectQuery<T, LimitOne> {
        unimplemented!()
    }

    pub fn offset(&self, offset: uint) -> SelectQuery<T, L> {
        let mut query = self.clone();
        query.offset = Some(offset);
        query
    }

}

impl<T: Clone, L: Clone> ToSelectQuery for SelectQuery<T, L> {
    
}

pub type BoxedSelectQuery = Box<ToSelectQuery + Send + Sync>;
pub type RcSelectQuery = Arc<BoxedSelectQuery>;
