
use time::Timespec;
use serialize::json::Json;

use to_sql::{ToPredicateValue};
use expression::{ToExpression, RawExpr};
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
        impl<T: ToExpression<$v> + Send + Sync + ToPredicateValue> Predicate for IsPredicate<$field, T> { }
        impl<T: ToExpression<$v> + Send + Sync + ToPredicateValue> ToIsPredicate<$field, T> for $field {
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
impl_for!(RawExpr, RawExpr)