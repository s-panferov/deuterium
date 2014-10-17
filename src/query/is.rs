
use serialize::json::Json;
use time::Timespec;

use query::{Query, RcQuery};
use expression::{RawExpression};
use field::{
    NamedField,

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
use to_sql::ToSql;

#[deriving(Send, Clone)]
pub struct IsQuery<F, T> {
    pub field: F,
    pub value: T
}

pub trait ToIsQuery<F, T> {
    fn is(&self, val: T) -> RcQuery;
}

macro_rules! is_methods(
    ($v:ty) => (
        fn is(&self, val: $v) -> RcQuery {
            IsQuery {
                field: self.clone(),
                value: val
            }.upcast()
        }
    )
)

macro_rules! impl_for(
    ($field:ty, $v:ty) => (
        impl Query for IsQuery<$field, $v> { }

        impl ToIsQuery<$field, $v> for $field {
            is_methods!($v)   
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

impl Query for IsQuery<RawExpression, RawExpression> { }
impl ToIsQuery<RawExpression, RawExpression> for RawExpression {
    is_methods!(RawExpression)
}