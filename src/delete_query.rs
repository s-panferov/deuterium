use std::mem;

use select_query::{Queryable, Select, SelectOnly, SelectAll, LimitMany, NoResult};
use from::{From, Table, RcTable, RcFrom};
use predicate::{RcPredicate};
use expression::{Expression, UntypedExpression};

pub trait Deletable<M>: Table { 
    fn delete(&self) -> DeleteQuery<(), NoResult, M> {
        DeleteQuery::new(self)
    }
}

macro_rules! returning_for(
    ($query:ident) => (
        impl<T, L, M> $query<T, L, M> {
            pub fn returning_1<T: Clone>(mut self, field: &Expression<T>) -> $query<(T), LimitMany, M> {
                self.returning = Some(SelectOnly(vec![field.upcast_expression()]));
                unsafe{ mem::transmute(self) }
            }

            pub fn returning_2<T1: Clone, T2: Clone>(mut self, field1: &Expression<T1>, field2: &Expression<T2>) -> $query<(T1, T2), LimitMany, M> {
                self.returning = Some(SelectOnly(vec![field1.upcast_expression(), field2.upcast_expression()]));
                unsafe{ mem::transmute(self) }
            }

            pub fn returning(mut self, fields: &[&UntypedExpression]) -> $query<(), LimitMany, M> {
                self.returning = Some(SelectOnly(fields.iter().map(|f| f.upcast_expression()).collect()));
                unsafe{ mem::transmute(self) }
            }

            pub fn returning_all(mut self) -> $query<(), LimitMany, M> {
                self.returning = Some(SelectAll);
                unsafe{ mem::transmute(self) }
            }
            
            pub fn no_returning(mut self) -> $query<(), NoResult, M> {
                self.returning = None;
                unsafe{ mem::transmute(self) }
            }
        }
    )
)

#[deriving(Clone)]
pub struct DeleteQuery<T, L, M> {
    pub only: bool,
    pub table: RcTable,
    pub using: Option<Vec<RcFrom>>,
    pub where_: Option<RcPredicate>,
    pub all: bool,
    pub returning: Option<Select>
}

impl<T, L, M> DeleteQuery<T, L, M> {
    pub fn new(table: &Table) -> DeleteQuery<T, L, M> {
        DeleteQuery {
            only: false,
            table: table.upcast_table(),
            using: None,
            where_: None,
            all: false,
            returning: None
        }
    }

    pub fn only(mut self) -> DeleteQuery<T, L, M> {
        self.only = true;
        self
    }

    pub fn using(mut self, using: &From) -> DeleteQuery<T, L, M> {
        if self.using.is_none() {
            self.using = Some(vec![])
        }

        self.using.as_mut().unwrap().push(using.upcast_from());
        self
    }

    pub fn all(mut self) -> DeleteQuery<T, L, M> {
        self.where_ = None;
        self.all = true;
        self
    }
}

returning_for!(DeleteQuery)

impl<T:Clone, L:Clone, M:Clone> Queryable for DeleteQuery<T, L, M> { 
    fn get_where(&self) -> &Option<RcPredicate> { &self.where_ }
    fn set_where(&mut self, predicate: RcPredicate) { self.where_ = Some(predicate); }
    fn unset_where(&mut self) { self.where_ = None; }
}

