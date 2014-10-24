
use predicate::{Predicate, RcPredicate};
use raw_expression::{RawExpression, RawExpressionComparable};

use field::{
    I8Field, I8Comparable,
    I16Field, I16Comparable,
    I32Field, I32Comparable,
    I64Field, I64Comparable,
    F32Field, F32Comparable,
    F64Field, F64Comparable,
    TimespecField, TimespecComparable
};

#[deriving(Clone)]
pub enum InRangeBounds {
    ExcludeBoth,
    IncludeBoth,
    ExcludeRight,
    ExcludeLeft
}

#[deriving(Send, Clone)]
pub struct InRangePredicate<F, T> {
    pub field: F,
    pub from: T,
    pub to: T,
    pub bounds: InRangeBounds
}

pub trait ToInRangePredicate<F, T> {
    fn in_range(&self, from: T, to: T) -> RcPredicate;
    fn in_range_exclude_left(&self, from: T, to: T) -> RcPredicate;
    fn in_range_exclude_right(&self, from: T, to: T) -> RcPredicate;
    fn in_range_exclude(&self, from: T, to: T) -> RcPredicate;
}

macro_rules! in_range_methods(
    ($v:ty) => (
        fn in_range(&self, from: $v, to: $v) -> RcPredicate {
            InRangePredicate {
                field: self.clone(),
                from: from,
                to: to,
                bounds: IncludeBoth
            }.upcast()
        }

        fn in_range_exclude_left(&self, from: $v, to: $v) -> RcPredicate {
            InRangePredicate {
                field: self.clone(),
                from: from,
                to: to,
                bounds: ExcludeLeft
            }.upcast()
        }

        fn in_range_exclude_right(&self, from: $v, to: $v) -> RcPredicate {
            InRangePredicate {
                field: self.clone(),
                from: from,
                to: to,
                bounds: ExcludeRight
            }.upcast()
        }

        fn in_range_exclude(&self, from: $v, to: $v) -> RcPredicate {
            InRangePredicate {
                field: self.clone(),
                from: from,
                to: to,
                bounds: ExcludeBoth
            }.upcast()
        }
    )
)

macro_rules! impl_for(
    ($field:ty, $v:ident) => (
        impl<T: $v> Predicate for InRangePredicate<$field, T> { }

        impl<T: $v> ToInRangePredicate<$field, T> for $field {
            in_range_methods!(T)    
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
