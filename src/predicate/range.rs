use time::Timespec;

use predicate::{Predicate, RcPredicate};
use sql::{ToPredicateValue};
use expression::{ToExpression};

use expression::{RawExpr};

use field::{
    I8Field,
    I16Field,
    I32Field,
    I64Field,
    F32Field,
    F64Field,
    TimespecField,

    OptionalI8Field,
    OptionalI16Field,
    OptionalI32Field,
    OptionalI64Field,
    OptionalF32Field,
    OptionalF64Field,
    OptionalTimespecField,
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
    ($field:ty, $v:ty) => (
        impl<T: ToExpression<$v> + Send + Sync + ToPredicateValue + Clone> Predicate for InRangePredicate<$field, T> { }

        impl<T: ToExpression<$v> + Send + Sync + ToPredicateValue + Clone> ToInRangePredicate<$field, T> for $field {
            in_range_methods!(T)    
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

impl_for!(OptionalI8Field, Option<i8>)
impl_for!(OptionalI16Field, Option<i16>)
impl_for!(OptionalI32Field, Option<i32>)
impl_for!(OptionalI64Field, Option<i64>)
impl_for!(OptionalF32Field, Option<f32>)
impl_for!(OptionalF64Field, Option<f64>)
impl_for!(OptionalTimespecField, Option<Timespec>)

impl_for!(RawExpr, RawExpr)
