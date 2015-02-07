use std::mem;
use std::rc;

use super::select_query;
use super::insert_query::{self, ToInsertValue};
use super::from;
use super::predicate;
use super::expression;
use super::sql;
use super::field;

pub trait FieldUpd: sql::ToSql {
    fn upcast_field_update(&self) -> SharedFieldUpdate;
}

#[derive(Clone)]
pub struct FieldUpdate<F, T> {
    pub field: F,
    pub value: insert_query::InsertValue<T>
}

impl<F, T> FieldUpdate<F, T> {
    pub fn get_field(&self) -> &F {
        &self.field
    }    

    pub fn get_value(&self) -> &insert_query::InsertValue<T> {
        &self.value
    }
}

pub type BoxedFieldUpdate = Box<FieldUpd + 'static>;
pub type SharedFieldUpdate = rc::Rc<BoxedFieldUpdate>;

impl<F, T> FieldUpd for FieldUpdate<F, T>
    where F: Clone + sql::ToPredicateValue + 'static,
          T: Clone + sql::ToPredicateValue + 'static {
    fn upcast_field_update(&self) -> SharedFieldUpdate {
        rc::Rc::new(Box::new(self.clone()))
    }
}

pub trait ToFieldUpdate<F, T> {
    fn set<B: expression::ToExpression<T>>(&self, val: &B) -> FieldUpdate<F, T>;
    fn set_default(&self) -> FieldUpdate<F, T>;
}

impl<T> ToFieldUpdate<field::NamedField<T>, T> for field::NamedField<T> where T: Clone {
    fn set<B: expression::ToExpression<T>>(&self, val: &B) -> FieldUpdate<field::NamedField<T>, T> {
        FieldUpdate {
            field: self.clone(),
            value: val.as_expr().to_insert_val()
        }
    }

    fn set_default(&self) -> FieldUpdate<field::NamedField<T>, T> {
        FieldUpdate {
            field: self.clone(),
            value: insert_query::InsertValue::Default
        }
    }
}

impl ToFieldUpdate<expression::RawExpression, expression::RawExpression> for expression::RawExpression {
    fn set<B: expression::ToExpression<expression::RawExpression>>(&self, val: &B) -> FieldUpdate<expression::RawExpression, expression::RawExpression> {
        FieldUpdate {
            field: self.clone(),
            value: val.as_expr().to_insert_val()
        }
    }

    fn set_default(&self) -> FieldUpdate<expression::RawExpression, expression::RawExpression> {
        FieldUpdate {
            field: self.clone(),
            value: insert_query::InsertValue::Default
        }
    }
}

pub trait Updatable<M>: from::Table + Sized { 
    fn update(&self) -> UpdateQuery<(), select_query::NoResult, M> {
        UpdateQuery::new(self)
    }
}

#[derive(Clone)]
pub struct UpdateQuery<T, L, M> {
    only: bool,
    table: from::SharedTable,
    updates: Vec<SharedFieldUpdate>,
    from: Option<Vec<from::SharedFrom>>,
    where_: Option<predicate::SharedPredicate>,
    all: bool,
    returning: Option<select_query::Select>
}

impl<T, L, M> UpdateQuery<T, L, M> {
    pub fn is_only(&self) -> bool { self.only }
    pub fn is_all(&self) -> bool { self.all }

    pub fn get_table(&self) -> &from::SharedTable { &self.table }
    pub fn get_updates(&self) -> &Vec<SharedFieldUpdate> { &self.updates }
    pub fn get_from(&self) -> &Option<Vec<from::SharedFrom>> { &self.from }
    pub fn get_where(&self) -> &Option<predicate::SharedPredicate> { &self.where_ }
    pub fn get_returning(&self) -> &Option<select_query::Select> { &self.returning }
}

impl<T, L, M> UpdateQuery<T, L, M> {
    pub fn new(table: &from::Table) -> UpdateQuery<T, L, M> {
        UpdateQuery {
            only: false,
            table: table.upcast_table(),
            updates: vec![],
            from: None,
            where_: None,
            all: false,
            returning: None
        }
    }

    pub fn only(mut self) -> UpdateQuery<T, L, M> {
        self.only = true;
        self
    }

    pub fn from(mut self, from: &from::From) -> UpdateQuery<T, L, M> {
        if self.from.is_none() {
            self.from = Some(vec![])
        }

        self.from.as_mut().unwrap().push(from.upcast_from());
        self
    }

    pub fn field<F: FieldUpd>(mut self, update: F) -> UpdateQuery<T, L, M> {
        self.updates.push(update.upcast_field_update());
        self
    }

    pub fn all(mut self) -> UpdateQuery<T, L, M> {
        self.where_ = None;
        self.all = true;
        self
    }
}

returning_for!(UpdateQuery);

impl<T:Clone, L:Clone, M:Clone> select_query::Queryable for UpdateQuery<T, L, M> { 
    fn get_where(&self) -> &Option<predicate::SharedPredicate> { &self.where_ }
    fn set_where(&mut self, predicate: predicate::SharedPredicate) { self.where_ = Some(predicate); }
    fn unset_where(&mut self) { self.where_ = None; }
}
