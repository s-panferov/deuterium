
use time::Timespec;
use serialize::json::Json;
use uuid::Uuid;

use sql::{ToPredicateValue};
use expression::{ToExpression};
#[cfg(feature = "raw_expr")]
use expression::{RawExpr};
use predicate::{Predicate, RcPredicate};
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

#[deriving(Send, Clone)]
pub struct IsPredicate<F, T> {
    pub field: F,
    pub value: T
}

pub trait ToIsPredicate<F, T> {
    fn is(&self, val: T) -> RcPredicate;
}

macro_rules! is_methods(
    ($v:ty) => (
        fn is(&self, val: $v) -> RcPredicate {
            IsPredicate {
                field: self.clone(),
                value: val
            }.upcast()
        }
    )
)

macro_rules! impl_for(
    ($field:ty, $v:ty) => (
        impl<T> Predicate for IsPredicate<$field, T> where T: Send + Sync + ToPredicateValue + Clone { }
        impl<T> ToIsPredicate<$field, T> for $field where T: ToExpression<$v> + Send + Sync + ToPredicateValue + Clone {
            is_methods!(T) 
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

#[cfg(feature = "raw_expr")]
impl_for!(RawExpr, RawExpr)