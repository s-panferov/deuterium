
use std::sync::Arc;

use select_query::{Queryable};
use from::{From, Table, RcTable, RcFrom};
use predicate::{RcPredicate};
use raw_expression::{RawExpression, RawExpressionComparable};
use to_sql::{ToSql, ToPredicateValue};
use field::{
    BoolField, BoolComparable,
    I8Field, I8Comparable,
    I16Field, I16Comparable,
    I32Field, I32Comparable,
    I64Field, I64Comparable,
    F32Field, F32Comparable,
    F64Field, F64Comparable,
    StringField, StringComparable,
    ByteListField, ByteListComparable,
    JsonField, JsonComparable,
    TimespecField, TimespecComparable
};

pub trait FieldUpd: ToSql {
    fn upcast_field_update(&self) -> RcFieldUpdate;
}

#[deriving(Send, Sync, Clone)]
pub struct FieldUpdate<F, T> {
    pub field: F,
    pub value: T
}

impl<F, T> FieldUpdate<F, T> {
    pub fn get_field(&self) -> &F {
        &self.field
    }    

    pub fn get_value(&self) -> &T {
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
    fn set(&self, val: T) -> FieldUpdate<F, T>;
}

macro_rules! set_methods(
    ($field:ty, $v:ident) => (
        fn set(&self, val: $v) -> FieldUpdate<$field, $v> {
            FieldUpdate {
                field: self.clone(),
                value: val
            }
        }
    )
)

macro_rules! impl_for(
    ($field:ty, $v:ident) => (
        impl<T: $v> ToFieldUpdate<$field, T> for $field {
            set_methods!($field, T) 
        }
    )
)

impl_for!(BoolField, BoolComparable)
impl_for!(I8Field, I8Comparable)
impl_for!(I16Field, I16Comparable)
impl_for!(I32Field, I32Comparable)
impl_for!(I64Field, I64Comparable)
impl_for!(F32Field, F32Comparable)
impl_for!(F64Field, F64Comparable)
impl_for!(StringField, StringComparable)
impl_for!(ByteListField, ByteListComparable)
impl_for!(JsonField, JsonComparable)
impl_for!(TimespecField, TimespecComparable)
impl_for!(RawExpression, RawExpressionComparable)

pub trait Updatable: Table { 
    fn update(&self) -> UpdateQuery {
        UpdateQuery::new(self)
    }
}

#[deriving(Clone)]
pub struct UpdateQuery {
    pub only: bool,
    pub table: RcTable,
    pub updates: Vec<RcFieldUpdate>,
    pub from: Option<Vec<RcFrom>>,
    pub where_: Option<RcPredicate>
}

impl UpdateQuery {
    pub fn new(table: &Table) -> UpdateQuery {
        UpdateQuery {
            only: false,
            table: table.upcast_table(),
            updates: vec![],
            from: None,
            where_: None
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
}

impl Queryable for UpdateQuery { 
    fn get_where(&self) -> &Option<RcPredicate> { &self.where_ }
    fn set_where(&mut self, predicate: RcPredicate) { self.where_ = Some(predicate); }
    fn unset_where(&mut self) { self.where_ = None; }
}
