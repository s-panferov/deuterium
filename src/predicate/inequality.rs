

use serialize::json::Json;
use time::Timespec;

use predicate::{Predicate, RcPredicate};
use expression::{RawExpression, RawExpressionComparable};
use field::{
    NamedField,

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
use to_sql::{ToPredicateValue};

#[deriving(Clone)]
pub enum Inequality {
    LessThan,
    LessThanEqual,
    GreaterThan,
    GreaterTranEqual
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
                inequality: LessThanEqual
            }.upcast()
        }
    )
)

macro_rules! impl_for(
    ($field:ty, $v:ident) => (
        impl<T: $v> Predicate for InequalityPredicate<$field, T> { }

        impl<T: $v> ToInequalityPredicate<$field, T> for $field {
            inequality_methods!(T)    
        }
    )
)

impl_for!(I8Field, I8Comparable)
impl_for!(I16Field, I16Comparable)
impl_for!(I32Field, I32Comparable)
impl_for!(I64Field, I64Comparable)
impl_for!(F32Field, F32Comparable)
impl_for!(F64Field, F64Comparable)
impl_for!(TimespecField, TimespecComparable)
impl_for!(RawExpression, RawExpressionComparable)