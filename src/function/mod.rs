
use std::rc::Rc;
use time::Timespec;

use expression::{Expression, BoxedExpression, RcExpression, UntypedExpression};
use sql::{ToSql};
use field::{
    NamedField,
    I8Field,
    I16Field,
    I32Field,
    I64Field,
    F32Field,
    F64Field,
    StringField,
    TimespecField
};

macro_rules! agg_func {
    ($foo:ident, $foo_arg:ident, $foo_low:ident) => (
        pub trait $foo_arg<R: Clone, T: Clone>: Clone + Expression<T> {
            fn $foo_low(&self) -> $foo<R, T, Self> {
                $foo::new(self.clone())
            }
        }

        #[derive(Clone)]
        pub struct $foo<R, T, E: $foo_arg<R, T>> {
            pub expression: E
        }

        #[allow(dead_code)]
        impl<R: Clone, T: Clone, E: $foo_arg<R, T> + 'static> $foo<R, T, E> {
            pub fn new(expr: E) -> $foo<R, T, E> {
                $foo {
                    expression: expr.clone()
                }
            }
        }

        impl<R: Clone, T: Clone, E: $foo_arg<R, T> + 'static> UntypedExpression for $foo<R, T, E> {
            fn expression_as_sql(&self) -> &ToSql {
                self
            }

            fn upcast_expression(&self) -> RcExpression {
                Rc::new(Box::new(self.clone()) as BoxedExpression)
            }
        }

        impl<R: Clone, T: Clone, E: $foo_arg<R, T>  + 'static> Expression<R> for $foo<R, T, E> { }
    )
}

agg_func!(Min, MinArg, min);

impl MinArg<i8, i8> for I8Field {}
impl MinArg<i16, i16> for I16Field {}
impl MinArg<i32, i32> for I32Field {}
impl MinArg<i64, i64> for I64Field {}
impl MinArg<f32, f32> for F32Field {}
impl MinArg<f64, f64> for F64Field {}
impl MinArg<String, String> for StringField {}
impl MinArg<Timespec, Timespec> for TimespecField {}

agg_func!(Max, MaxArg, max);

impl MaxArg<i8, i8> for I8Field {}
impl MaxArg<i16, i16> for I16Field {}
impl MaxArg<i32, i32> for I32Field {}
impl MaxArg<i64, i64> for I64Field {}
impl MaxArg<f32, f32> for F32Field {}
impl MaxArg<f64, f64> for F64Field {}
impl MaxArg<String, String> for StringField {}
impl MaxArg<Timespec, Timespec> for TimespecField {}

agg_func!(Sum, SumArg, sum);

impl SumArg<i64, i8> for I8Field {}
impl SumArg<i64, i16> for I16Field {}
impl SumArg<i64, i32> for I32Field {}
impl SumArg<i64, i64> for I64Field {}
impl SumArg<f64, f32> for F32Field {}
impl SumArg<f64, f64> for F64Field {}

agg_func!(Avg, AvgArg, avg);

impl AvgArg<i8, i8> for I8Field {}
impl AvgArg<i16, i16> for I16Field {}
impl AvgArg<i32, i32> for I32Field {}
impl AvgArg<i64, i64> for I64Field {}
impl AvgArg<f32, f32> for F32Field {}
impl AvgArg<f64, f64> for F64Field {}

agg_func!(Count, CountArg, count);

impl<T: Clone> CountArg<i64, T> for NamedField<T> {}

#[derive(Clone, Copy)]
pub struct CountAll;

impl UntypedExpression for CountAll {
    fn expression_as_sql(&self) -> &ToSql {
        self
    }

    fn upcast_expression(&self) -> RcExpression {
        Rc::new(Box::new(self.clone()) as BoxedExpression)
    }
}

impl Expression<i64> for CountAll { }