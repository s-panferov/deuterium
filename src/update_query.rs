use std::mem;

use std::rc::Rc;

use select_query::{Queryable, Select, LimitMany, NoResult};
use from::{From, Table, RcTable, RcFrom};
use predicate::{RcPredicate};
use expression::{Expression, UntypedExpression};

use expression::{
    RawExpr,
    ToExprValue,
    ExprValue, 
    ToExpression,
};

use sql::{ToSql, ToPredicateValue};
use field::{
    NamedField,
};

pub trait FieldUpd: ToSql {
    fn upcast_field_update(&self) -> RcFieldUpdate;
}

#[derive(Clone)]
pub struct FieldUpdate<F, T> {
    pub field: F,
    pub value: ExprValue<T>
}

impl<F, T> FieldUpdate<F, T> {
    pub fn get_field(&self) -> &F {
        &self.field
    }    

    pub fn get_value(&self) -> &ExprValue<T> {
        &self.value
    }
}

pub type BoxedFieldUpdate = Box<FieldUpd + 'static>;
pub type RcFieldUpdate = Rc<BoxedFieldUpdate>;

impl<F: Clone + ToPredicateValue + 'static, T: Clone + ToPredicateValue + 'static> FieldUpd for FieldUpdate<F, T> {
    fn upcast_field_update(&self) -> RcFieldUpdate {
        Rc::new(Box::new(self.clone()) as BoxedFieldUpdate)
    }
}

pub trait ToFieldUpdate<F, T> {
    fn set<B: ToExpression<T>>(&self, val: &B) -> FieldUpdate<F, T>;
    fn set_default(&self) -> FieldUpdate<F, T>;
}

impl<T> ToFieldUpdate<NamedField<T>, T> for NamedField<T> where T: Clone {
    fn set<B: ToExpression<T>>(&self, val: &B) -> FieldUpdate<NamedField<T>, T> {
        FieldUpdate {
            field: self.clone(),
            value: val.as_expr().to_expr_val()
        }
    }

    fn set_default(&self) -> FieldUpdate<NamedField<T>, T> {
        FieldUpdate {
            field: self.clone(),
            value: ExprValue::Default
        }
    }
}

impl ToFieldUpdate<RawExpr, RawExpr> for RawExpr {
    fn set<B: ToExpression<RawExpr>>(&self, val: &B) -> FieldUpdate<RawExpr, RawExpr> {
        FieldUpdate {
            field: self.clone(),
            value: val.as_expr().to_expr_val()
        }
    }

    fn set_default(&self) -> FieldUpdate<RawExpr, RawExpr> {
        FieldUpdate {
            field: self.clone(),
            value: ExprValue::Default
        }
    }
}

pub trait Updatable<M>: Table { 
    fn update(&self) -> UpdateQuery<(), NoResult, M> {
        UpdateQuery::new(self)
    }
}

#[derive(Clone)]
pub struct UpdateQuery<T, L, M> {
    pub only: bool,
    pub table: RcTable,
    pub updates: Vec<RcFieldUpdate>,
    pub from: Option<Vec<RcFrom>>,
    pub where_: Option<RcPredicate>,
    pub all: bool,
    pub returning: Option<Select>
}

impl<T, L, M> UpdateQuery<T, L, M> {
    pub fn new(table: &Table) -> UpdateQuery<T, L, M> {
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

    pub fn from(mut self, from: &From) -> UpdateQuery<T, L, M> {
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

impl<T:Clone, L:Clone, M:Clone> Queryable for UpdateQuery<T, L, M> { 
    fn get_where(&self) -> &Option<RcPredicate> { &self.where_ }
    fn set_where(&mut self, predicate: RcPredicate) { self.where_ = Some(predicate); }
    fn unset_where(&mut self) { self.where_ = None; }
}
