use select_query::{Queryable};
use from::{From, Table, RcTable, RcFrom};
use predicate::{RcPredicate};

pub trait Deletable: Table { 
    fn delete(&self) -> DeleteQuery {
        DeleteQuery::new(self)
    }
}

// TODO: RETURNING

#[deriving(Clone)]
pub struct DeleteQuery {
    pub only: bool,
    pub table: RcTable,
    pub using: Option<Vec<RcFrom>>,
    pub where_: Option<RcPredicate>,
    pub all: bool
}

impl DeleteQuery {
    pub fn new(table: &Table) -> DeleteQuery {
        DeleteQuery {
            only: false,
            table: table.upcast_table(),
            using: None,
            where_: None,
            all: false
        }
    }

    pub fn only(mut self) -> DeleteQuery {
        self.only = true;
        self
    }

    pub fn using(mut self, using: &From) -> DeleteQuery {
        if self.using.is_none() {
            self.using = Some(vec![])
        }

        self.using.as_mut().unwrap().push(using.upcast_from());
        self
    }

    pub fn all(mut self) -> DeleteQuery {
        self.where_ = None;
        self.all = true;
        self
    }
}

impl Queryable for DeleteQuery { 
    fn get_where(&self) -> &Option<RcPredicate> { &self.where_ }
    fn set_where(&mut self, predicate: RcPredicate) { self.where_ = Some(predicate); }
    fn unset_where(&mut self) { self.where_ = None; }
}
