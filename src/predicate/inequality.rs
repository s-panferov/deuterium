
use time::Timespec;

use predicate::{Predicate, RcPredicate};
use expression::{ToExpression};
#[cfg(feature = "raw_expr")]
use expression::{RawExpr};
use field::{
    I8Field,
    I16Field,
    I32Field,
    I64Field,
    F32Field,
    F64Field,
    TimespecField,
};

use sql::{ToPredicateValue};

#[deriving(Clone)]
pub enum Inequality {
    LessThan,
    LessThanEqual,
    GreaterThan,
    GreaterThanEqual
}

#[deriving(Send, Clone)]
pub struct InequalityPredicate<F, T> {
    pub field: F,
    pub value: T,
    pub inequality: Inequality
}

pub trait ToInequalityPredicate<F, T> {
    fn lt(&self, val: T) -> RcPredicate;
    fn lte(&self, val: T) -> RcPredicate;
    fn gt(&self, val: T) -> RcPredicate;
    fn gte(&self, val: T) -> RcPredicate;
}

macro_rules! inequality_methods(
    ($v:ty) => (
        fn lt(&self, val: $v) -> RcPredicate {
            InequalityPredicate {
                field: self.clone(),
                value: val,
                inequality: LessThan
            }.upcast()
        }

        fn lte(&self, val: $v) -> RcPredicate {
            InequalityPredicate {
                field: self.clone(),
                value: val,
                inequality: LessThanEqual
            }.upcast()
        }

        fn gt(&self, val: $v) -> RcPredicate {
            InequalityPredicate {
                field: self.clone(),
                value: val,
                inequality: GreaterThan
            }.upcast()
        }

        fn gte(&self, val: $v) -> RcPredicate {
            InequalityPredicate {
                field: self.clone(),
                value: val,
                inequality: GreaterThanEqual
            }.upcast()
        }
    )
)

macro_rules! impl_for(
    ($field:ty, $v:ident) => (
        impl<T: ToExpression<$v> + Send + Sync + ToPredicateValue> Predicate for InequalityPredicate<$field, T> { }

        impl<T: ToExpression<$v> + Send + Sync + ToPredicateValue> ToInequalityPredicate<$field, T> for $field {
            inequality_methods!(T)    
        }
    )
)

impl_for!(I8Field, i8)
impl_for!(I16Field, i16)
impl_for!(I32Field, i32)
impl_for!(I64Field, i64)
impl_for!(F32Field, f32)
impl_for!(F64Field, f64)
impl_for!(TimespecField, Timespec)

#[cfg(feature = "raw_expr")]
impl_for!(RawExpr, RawExpr)