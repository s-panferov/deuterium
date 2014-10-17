
use serialize::json::Json;
use time::Timespec;

use {Null};
use query::{Query, RcQuery};
use expression::{RawExpression, RawExpressionComparable};
use field::{
    Field, NamedField,

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

use to_sql::ToSql;

#[deriving(Send, Clone)]
pub struct IsNullQuery<F> {
    pub field: F,
    pub null: bool
}

pub trait ToIsNullQuery {
    fn is_null(&self) -> RcQuery;
    fn not_null(&self) -> RcQuery;
}

macro_rules! impl_for(
    ($f:ty) => (

        impl Query for IsNullQuery<$f> { }
        impl ToIsNullQuery for $f {
            fn is_null(&self) -> RcQuery {
                IsNullQuery {
                    field: self.clone(),
                    null: true
                }.upcast()
            }

            fn not_null(&self) -> RcQuery {
                IsNullQuery {
                    field: self.clone(),
                    null: false
                }.upcast()
            }
        }

    )
)

impl_for!(BoolField)
impl_for!(I8Field)
impl_for!(I16Field)
impl_for!(I32Field)
impl_for!(I64Field)
impl_for!(F32Field)
impl_for!(F64Field)
impl_for!(StringField)
impl_for!(ByteListField)
impl_for!(JsonField)
impl_for!(TimespecField)
impl_for!(RawExpression)

