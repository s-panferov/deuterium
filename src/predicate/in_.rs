use time::Timespec;

use predicate::{Predicate, RcPredicate};
use expression::{ToListExpression, RawExpr};
use sql::{ToPredicateValue};

use field::{
    I8Field,
    I16Field,
    I32Field,
    I64Field,
    F32Field,
    F64Field,
    StringField,
    TimespecField,
};

#[deriving(Send, Clone)]
pub struct InPredicate<F, T> {
    pub field: F,
    pub values: T
}

pub trait ToInPredicate<F, T> {
    fn in_(&self, val: T) -> RcPredicate;
}

macro_rules! in_methods(
    ($v:ty) => (
        fn in_(&self, values: $v) -> RcPredicate {
            InPredicate {
                field: self.clone(),
                values: values
            }.upcast()
        }
    )
)

macro_rules! impl_for(
    ($field:ty, $v:ty) => (
        impl<T: ToListExpression<$v> + Send + Sync + ToPredicateValue> Predicate for InPredicate<$field, T> { }

        impl<T: ToListExpression<$v> + Send + Sync + ToPredicateValue> ToInPredicate<$field, T> for $field {
            in_methods!(T)   
        }
    )
)

impl_for!(I8Field, i8)
impl_for!(I16Field, i16)
impl_for!(I32Field, i32)
impl_for!(I64Field, i64)
impl_for!(F32Field, f32)
impl_for!(F64Field, f64)
impl_for!(StringField, String)
impl_for!(TimespecField, Timespec)
impl_for!(RawExpr, RawExpr)