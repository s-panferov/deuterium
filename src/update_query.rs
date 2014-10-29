
use std::sync::Arc;

use select_query::{Queryable};
use from::{From, Table, RcTable, RcFrom};
use predicate::{RcPredicate};

use serialize::json::Json;
use time::Timespec;

use expression::{
    RawExpr,
    ToExprValue,
    ExprValue, 
    DefaultValue,
    ToExpression,
};

use to_sql::{ToSql, ToPredicateValue};
use field::{
    BoolField,
    I8Field,
    I16Field,
    I32Field,
    I64Field,
    F32Field,
    F64Field,
    StringField,
    ByteListField,
    JsonField,
    TimespecField,
};

pub trait FieldUpd: ToSql {
    fn upcast_field_update(&self) -> RcFieldUpdate;
}

#[deriving(Send, Sync, Clone)]
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

pub type BoxedFieldUpdate = Box<FieldUpd + Send + Sync>;
pub type RcFieldUpdate = Arc<BoxedFieldUpdate>;

impl<F: Clone + Send + Sync + ToPredicateValue, T: Clone + Send + Sync + ToPredicateValue> FieldUpd for FieldUpdate<F, T> {
    fn upcast_field_update(&self) -> RcFieldUpdate {
        Arc::new(box self.clone() as BoxedFieldUpdate)
    }
}

pub trait ToFieldUpdate<F, T> {
    fn set<B: ToExpression<T>>(&self, val: &B) -> FieldUpdate<F, T>;
    fn set_default(&self) -> FieldUpdate<F, T>;
}

macro_rules! set_methods(
    ($field:ty, $v:ty) => (
        fn set<B: ToExpression<$v>>(&self, val: &B) -> FieldUpdate<$field, $v> {
            FieldUpdate {
                field: self.clone(),
                value: val.as_expr().to_expr_val()
            }
        }

        fn set_default(&self) -> FieldUpdate<$field, $v> {
            FieldUpdate {
                field: self.clone(),
                value: DefaultValue
            }
        }
    )
)

macro_rules! impl_for(
    ($field:ty, $v:ty) => (
        impl ToFieldUpdate<$field, $v> for $field {
            set_methods!($field, $v) 
        }
    )
)

impl_for!(BoolField, bool)
impl_for!(I8Field, i8)
impl_for!(I16Field, i16)
impl_for!(I32Field, i32)
impl_for!(I64Field, i64)
impl_for!(F32Field, f32)
impl_for!(F64Field, f64)
impl_for!(StringField, String)
impl_for!(ByteListField, Vec<u8>)
impl_for!(JsonField, Json)
impl_for!(TimespecField, Timespec)
impl_for!(RawExpr, RawExpr)

pub trait Updatable: Table { 
    fn update(&self) -> UpdateQuery {
        UpdateQuery::new(self)
    }
}

// TODO: RETURNING

#[deriving(Clone)]
pub struct UpdateQuery {
    pub only: bool,
    pub table: RcTable,
    pub updates: Vec<RcFieldUpdate>,
    pub from: Option<Vec<RcFrom>>,
    pub where_: Option<RcPredicate>,
    pub all: bool
}

impl UpdateQuery {
    pub fn new(table: &Table) -> UpdateQuery {
        UpdateQuery {
            only: false,
            table: table.upcast_table(),
            updates: vec![],
            from: None,
            where_: None,
            all: false
        }
    }

    pub fn only(mut self) -> UpdateQuery {
        self.only = true;
        self
    }

    pub fn from(mut self, from: &From) -> UpdateQuery {
        if self.from.is_none() {
            self.from = Some(vec![])
        }

        self.from.as_mut().unwrap().push(from.upcast_from());
        self
    }

    pub fn field<T: FieldUpd>(mut self, update: T) -> UpdateQuery {
        self.updates.push(update.upcast_field_update());
        self
    }

    pub fn all(mut self) -> UpdateQuery {
        self.where_ = None;
        self.all = true;
        self
    }
}

impl Queryable for UpdateQuery { 
    fn get_where(&self) -> &Option<RcPredicate> { &self.where_ }
    fn set_where(&mut self, predicate: RcPredicate) { self.where_ = Some(predicate); }
    fn unset_where(&mut self) { self.where_ = None; }
}
