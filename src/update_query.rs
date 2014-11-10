use std::mem;

use std::sync::Arc;

use select_query::{Queryable, Select, SelectOnly, SelectAll, LimitMany, NoResult};
use from::{From, Table, RcTable, RcFrom};
use predicate::{RcPredicate};
use expression::{Expression, UntypedExpression};

use serialize::json::Json;
use time::Timespec;
use uuid::Uuid;

use expression::{
    RawExpr,
    ToExprValue,
    ExprValue, 
    DefaultValue,
    ToExpression,
};

use sql::{ToSql, ToPredicateValue};
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
    UuidField,    

    OptionalBoolField,
    OptionalI8Field,
    OptionalI16Field,
    OptionalI32Field,
    OptionalI64Field,
    OptionalF32Field,
    OptionalF64Field,
    OptionalStringField,
    OptionalByteListField,
    OptionalJsonField,
    OptionalTimespecField,
    OptionalUuidField,
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
impl_for!(UuidField, Uuid)

impl_for!(OptionalBoolField, Option<bool>)
impl_for!(OptionalI8Field, Option<i8>)
impl_for!(OptionalI16Field, Option<i16>)
impl_for!(OptionalI32Field, Option<i32>)
impl_for!(OptionalI64Field, Option<i64>)
impl_for!(OptionalF32Field, Option<f32>)
impl_for!(OptionalF64Field, Option<f64>)
impl_for!(OptionalStringField, Option<String>)
impl_for!(OptionalByteListField, Option<Vec<u8>>)
impl_for!(OptionalJsonField, Option<Json>)
impl_for!(OptionalTimespecField, Option<Timespec>)
impl_for!(OptionalUuidField, Option<Uuid>)

impl_for!(RawExpr, RawExpr)

pub trait Updatable<M>: Table { 
    fn update(&self) -> UpdateQuery<(), NoResult, M> {
        UpdateQuery::new(self)
    }
}

#[deriving(Clone)]
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

returning_for!(UpdateQuery)

impl<T:Clone, L:Clone, M:Clone> Queryable for UpdateQuery<T, L, M> { 
    fn get_where(&self) -> &Option<RcPredicate> { &self.where_ }
    fn set_where(&mut self, predicate: RcPredicate) { self.where_ = Some(predicate); }
    fn unset_where(&mut self) { self.where_ = None; }
}
