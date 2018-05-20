use std::mem;
use std::marker;

use super::select_query;
use super::from;
use super::predicate;

pub trait Deletable<M>: from::Table + Sized {
    fn delete(&self) -> DeleteQuery<(), select_query::NoResult, M> {
        DeleteQuery::new(self)
    }
}

macro_rules! returning_for {
    ($query:ident) => (
        impl<T, L, M> $query<T, L, M> {
            pub fn returning_1<R: Clone>(mut self, field: &$crate::expression::Expression<R>) -> $query<(R), $crate::select_query::LimitMany, M> {
                self.returning = Some($crate::select_query::Select::Only(vec![field.upcast_expression()]));
                unsafe{ mem::transmute(self) }
            }

            pub fn returning_2<R1: Clone, R2: Clone>(mut self, field1: &$crate::expression::Expression<R1>, field2: &$crate::expression::Expression<R2>) -> $query<(R1, R2), $crate::select_query::LimitMany, M> {
                self.returning = Some($crate::select_query::Select::Only(vec![field1.upcast_expression(), field2.upcast_expression()]));
                unsafe{ mem::transmute(self) }
            }

            pub fn returning(mut self, fields: &[&$crate::expression::UntypedExpression]) -> $query<(), $crate::select_query::LimitMany, M> {
                self.returning = Some($crate::select_query::Select::Only(fields.iter().map(|f| f.upcast_expression()).collect()));
                unsafe{ mem::transmute(self) }
            }

            pub fn returning_all(mut self) -> $query<(), $crate::select_query::LimitMany, M> {
                self.returning = Some($crate::select_query::Select::All);
                unsafe{ mem::transmute(self) }
            }

            pub fn no_returning(mut self) -> $query<(), $crate::select_query::NoResult, M> {
                self.returning = None;
                unsafe{ mem::transmute(self) }
            }
        }
    )
}

#[derive(Clone, Debug)]
pub struct DeleteQuery<T, L, M> {
    only: bool,
    all: bool,
    table: from::SharedTable,
    using: Option<Vec<from::SharedFrom>>,
    where_: Option<predicate::SharedPredicate>,
    returning: Option<select_query::Select>,

    _marker_t: marker::PhantomData<T>,
    _marker_l: marker::PhantomData<L>,
    _marker_m: marker::PhantomData<M>,
}

impl<T, L, M> DeleteQuery<T, L, M> {

    pub fn is_only(&self) -> bool { self.only }
    pub fn is_all(&self) -> bool { self.all }
    pub fn get_table(&self) -> &from::SharedTable { &self.table }
    pub fn get_using(&self) -> &Option<Vec<from::SharedFrom>> { &self.using }
    pub fn get_where(&self) -> &Option<predicate::SharedPredicate> { &self.where_ }
    pub fn get_returning(&self) -> &Option<select_query::Select> { &self.returning }

    pub fn new(table: &from::Table) -> DeleteQuery<T, L, M> {
        DeleteQuery {
            only: false,
            table: table.upcast_table(),
            using: None,
            where_: None,
            all: false,
            returning: None,

            _marker_t: marker::PhantomData,
            _marker_l: marker::PhantomData,
            _marker_m: marker::PhantomData,
        }
    }

    pub fn only(mut self) -> DeleteQuery<T, L, M> {
        self.only = true;
        self
    }

    pub fn using(mut self, using: &from::From) -> DeleteQuery<T, L, M> {
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

returning_for!(DeleteQuery);

impl<T:Clone, L:Clone, M:Clone> select_query::Queryable for DeleteQuery<T, L, M> {
    fn get_where(&self) -> &Option<predicate::SharedPredicate> { &self.where_ }
    fn set_where(&mut self, predicate: predicate::SharedPredicate) { self.where_ = Some(predicate); }
    fn unset_where(&mut self) { self.where_ = None; }
}

