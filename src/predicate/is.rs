
// use time::Timespec;
// use serialize::json::Json;
// use uuid::Uuid;

use sql::{ToPredicateValue};
use expression::{ToExpression};

use expression::{RawExpr};
use predicate::{Predicate, RcPredicate};
use field::{
    NamedField,
    // BoolField,
    // I8Field,
    // I16Field,
    // I32Field,
    // I64Field,
    // F32Field,
    // F64Field,
    // StringField,
    // ByteListField,
    // JsonField,
    // TimespecField,
    // UuidField,

    // OptionalBoolField,
    // OptionalI8Field,
    // OptionalI16Field,
    // OptionalI32Field,
    // OptionalI64Field,
    // OptionalF32Field,
    // OptionalF64Field,
    // OptionalStringField,
    // OptionalByteListField,
    // OptionalJsonField,
    // OptionalTimespecField,
    // OptionalUuidField,
};

#[deriving(Send, Clone)]
pub struct IsPredicate<F, T> {
    pub field: F,
    pub value: T
}

pub trait ToIsPredicate<F, T> {
    fn is<B: ToExpression<T> + ToPredicateValue + Send + Sync + Clone>(&self, val: B) -> RcPredicate;
}

macro_rules! is_methods(
    ($v:ty) => (
        fn is<B: ToExpression<$v> + ToPredicateValue + Send + Sync + Clone>(&self, val: B) -> RcPredicate {
            IsPredicate {
                field: self.clone(),
                value: val
            }.upcast()
        }
    )
)

macro_rules! impl_for(
    ($field:ty, $v:ty) => (
        impl<T, B: ToExpression<T> + ToPredicateValue + Send + Sync + Clone> Predicate for IsPredicate<$field, B> where T: Send + Sync + ToPredicateValue + Clone { }
        impl<T> ToIsPredicate<$field, T> for $field where T: ToExpression<$v> + Send + Sync + ToPredicateValue + Clone {
            is_methods!(T) 
        }
    )
)


impl_for!(NamedField<T>, T)
impl_for!(RawExpr, RawExpr)