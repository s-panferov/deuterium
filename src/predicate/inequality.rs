use time;

use super::{ToAbstractPredicate};
use super::super::expression;
use super::super::field;
use super::super::sql;

#[derive(Clone, Copy)]
pub enum Inequality {
    LessThan,
    LessThanEqual,
    GreaterThan,
    GreaterThanEqual
}

#[derive(Clone)]
pub struct InequalityPredicate<F, T> {
    pub field: F,
    pub value: T,
    pub inequality: Inequality
}

pub trait ToInequalityPredicate<T> {
    fn lt<B>(&self, val: B) -> super::SharedPredicate
        where B: expression::ToExpression<T> + sql::ToPredicateValue + Clone + 'static;

    fn lte<B>(&self, val: B) -> super::SharedPredicate
        where B: expression::ToExpression<T> + sql::ToPredicateValue + Clone + 'static;

    fn gt<B>(&self, val: B) -> super::SharedPredicate
        where B: expression::ToExpression<T> + sql::ToPredicateValue + Clone + 'static;

    fn gte<B>(&self, val: B) -> super::SharedPredicate
        where B: expression::ToExpression<T> + sql::ToPredicateValue + Clone + 'static;
}

impl<F, T> super::Predicate for InequalityPredicate<F, T> 
    where F: sql::ToPredicateValue,
          T: sql::ToPredicateValue { }

macro_rules! impl_for {
    ($field:ty, $expr:ty) => (

        impl ToInequalityPredicate<$expr> for $field {
            fn lt<B>(&self, val: B) -> super::SharedPredicate
                where B: expression::ToExpression<$expr> + sql::ToPredicateValue + Clone + 'static {
                InequalityPredicate { field: self.clone(), value: val, inequality: Inequality::LessThan }.upcast()
            }

            fn lte<B>(&self, val: B) -> super::SharedPredicate
                where B: expression::ToExpression<$expr> + sql::ToPredicateValue + Clone + 'static {
                InequalityPredicate { field: self.clone(), value: val, inequality: Inequality::LessThanEqual }.upcast()
            }

            fn gt<B>(&self, val: B) -> super::SharedPredicate
                where B: expression::ToExpression<$expr> + sql::ToPredicateValue + Clone + 'static {
                InequalityPredicate { field: self.clone(), value: val, inequality: Inequality::GreaterThan }.upcast()
            }

            fn gte<B>(&self, val: B) -> super::SharedPredicate
                where B: expression::ToExpression<$expr> + sql::ToPredicateValue + Clone + 'static {
                InequalityPredicate { field: self.clone(), value: val, inequality: Inequality::GreaterThanEqual }.upcast()
            }
        }

    )
}

impl_for!(field::I8Field, i8);
impl_for!(field::I16Field, i16);
impl_for!(field::I32Field, i32);
impl_for!(field::I64Field, i64);
impl_for!(field::F32Field, f32);
impl_for!(field::F64Field, f64);
impl_for!(field::TimespecField, time::Timespec);

impl_for!(field::OptionalI8Field, Option<i8>);
impl_for!(field::OptionalI16Field, Option<i16>);
impl_for!(field::OptionalI32Field, Option<i32>);
impl_for!(field::OptionalI64Field, Option<i64>);
impl_for!(field::OptionalF32Field, Option<f32>);
impl_for!(field::OptionalF64Field, Option<f64>);
impl_for!(field::OptionalTimespecField, Option<time::Timespec>);

impl_for!(expression::RawExpr, expression::RawExpr);