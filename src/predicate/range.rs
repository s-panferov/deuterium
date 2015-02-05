use time::Timespec;

use predicate::{Predicate, ToAbstractPredicate, RcPredicate};
use sql::{ToPredicateValue};
use expression::{ToExpression};

#[derive(Clone, Copy)]
pub enum InRangeBounds {
    ExcludeBoth,
    IncludeBoth,
    ExcludeRight,
    ExcludeLeft
}

#[derive(Clone)]
pub struct InRangePredicate<F, T> {
    pub field: F,
    pub from: T,
    pub to: T,
    pub bounds: InRangeBounds
}

pub trait ToInRangePredicate<F, T> {
    fn in_range<F, T>(&self, from: B, to: B) -> RcPredicate 
        where B: ToExpression<T> + ToPredicateValue + Clone + 'static;
    fn in_range_exclude_left<F, T>(&self, from: B, to: B) -> RcPredicate 
        where B: ToExpression<T> + ToPredicateValue + Clone + 'static;
    fn in_range_exclude_right<F, T>(&self, from: B, to: B) -> RcPredicate 
        where B: ToExpression<T> + ToPredicateValue + Clone + 'static;
    fn in_range_exclude<F, T>(&self, from: B, to: B) -> RcPredicate 
        where B: ToExpression<T> + ToPredicateValue + Clone + 'static;
}

macro_rules! in_range_methods {
    ($v:ty) => (
        fn in_range(&self, from: $v, to: $v) -> RcPredicate {
            InRangePredicate {
                field: self.clone(),
                from: from,
                to: to,
                bounds: InRangeBounds::IncludeBoth
            }.upcast()
        }

        fn in_range_exclude_left(&self, from: $v, to: $v) -> RcPredicate {
            InRangePredicate {
                field: self.clone(),
                from: from,
                to: to,
                bounds: InRangeBounds::ExcludeLeft
            }.upcast()
        }

        fn in_range_exclude_right(&self, from: $v, to: $v) -> RcPredicate {
            InRangePredicate {
                field: self.clone(),
                from: from,
                to: to,
                bounds: InRangeBounds::ExcludeRight
            }.upcast()
        }

        fn in_range_exclude(&self, from: $v, to: $v) -> RcPredicate {
            InRangePredicate {
                field: self.clone(),
                from: from,
                to: to,
                bounds: InRangeBounds::ExcludeBoth
            }.upcast()
        }
    )
}

macro_rules! impl_for {
    ($field:ty, $v:ty) => (
        impl<T: ToExpression<$v> + ToPredicateValue + Clone> Predicate for InRangePredicate<$field, T> { }

        impl<T: ToExpression<$v> + ToPredicateValue + Clone> ToInRangePredicate<$field, T> for $field {
            in_range_methods!(T);  
        }
    )
}

impl<F, T> Predicate for InRangePredicate<F, T> 
    where F: ToPredicateValue,
          T: ToPredicateValue { }

impl<F, T> ToInRangePredicate<F, T> for F 
    where F: ToPredicateValue + ToExpression<T> + Clone + 'static,
          T: ToPredicateValue + Clone {
    
    fn in_range<B>(&self, from: B, to: B) -> RcPredicate {
        InRangePredicate {
            field: self.clone(),
            from: from,
            to: to,
            bounds: InRangeBounds::IncludeBoth
        }.upcast()
    }

    fn in_range_exclude_left(&self, from: $v, to: $v) -> RcPredicate {
        InRangePredicate {
            field: self.clone(),
            from: from,
            to: to,
            bounds: InRangeBounds::ExcludeLeft
        }.upcast()
    }

    fn in_range_exclude_right(&self, from: $v, to: $v) -> RcPredicate {
        InRangePredicate {
            field: self.clone(),
            from: from,
            to: to,
            bounds: InRangeBounds::ExcludeRight
        }.upcast()
    }

    fn in_range_exclude(&self, from: $v, to: $v) -> RcPredicate {
        InRangePredicate {
            field: self.clone(),
            from: from,
            to: to,
            bounds: InRangeBounds::ExcludeBoth
        }.upcast()
    }
}

impl_for!(I8Field, i8);
impl_for!(I16Field, i16);
impl_for!(I32Field, i32);
impl_for!(I64Field, i64);
impl_for!(F32Field, f32);
impl_for!(F64Field, f64);
impl_for!(TimespecField, Timespec);

impl_for!(OptionalI8Field, Option<i8>);
impl_for!(OptionalI16Field, Option<i16>);
impl_for!(OptionalI32Field, Option<i32>);
impl_for!(OptionalI64Field, Option<i64>);
impl_for!(OptionalF32Field, Option<f32>);
impl_for!(OptionalF64Field, Option<f64>);
impl_for!(OptionalTimespecField, Option<Timespec>);

impl_for!(RawExpr, RawExpr);
