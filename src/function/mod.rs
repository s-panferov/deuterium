use std::marker;
use std::{fmt, rc};
use time;

use expression;
use field;
use sql;

macro_rules! agg_func {
    ($foo:ident, $foo_arg:ident, $foo_low:ident) => (
        pub trait $foo_arg<R: Clone + 'static, T: Clone + 'static>: Clone + expression::Expression<T>+ 'static {
            fn $foo_low(&self) -> $foo<R, T, Self> {
                $foo::new(self.clone())
            }
        }

        #[derive(Clone, Debug)]
        pub struct $foo<R: Clone + 'static, T: Clone + 'static, E: $foo_arg<R, T>> {
            pub expression: E,

            _marker_r: marker::PhantomData<R>,
            _marker_t: marker::PhantomData<T>,
        }

        impl<R: Clone + 'static, T: Clone + 'static, E: $foo_arg<R, T> + 'static> $foo<R, T, E> {
            pub fn new(expr: E) -> $foo<R, T, E> {
                $foo {
                    expression: expr.clone(),

                    _marker_r: marker::PhantomData,
                    _marker_t: marker::PhantomData,
                }
            }
        }

        impl<R: Clone + 'static + fmt::Debug, T: Clone + 'static + fmt::Debug, E: $foo_arg<R, T> + 'static> expression::UntypedExpression for $foo<R, T, E> {
            fn expression_as_sql(&self) -> &sql::ToSql {
                self
            }

            fn upcast_expression(&self) -> expression::SharedExpression {
                rc::Rc::new(Box::new(self.clone()) as expression::BoxedExpression)
            }
        }

        impl<R: Clone + 'static + fmt::Debug, T: Clone + 'static + fmt::Debug, E: $foo_arg<R, T>  + 'static> expression::Expression<R> for $foo<R, T, E> { }
    )
}

agg_func!(Min, MinArg, min);

impl MinArg<i8, i8> for field::I8Field {}
impl MinArg<i16, i16> for field::I16Field {}
impl MinArg<i32, i32> for field::I32Field {}
impl MinArg<i64, i64> for field::I64Field {}
impl MinArg<f32, f32> for field::F32Field {}
impl MinArg<f64, f64> for field::F64Field {}
impl MinArg<String, String> for field::StringField {}
impl MinArg<time::Timespec, time::Timespec> for field::TimespecField {}

agg_func!(Max, MaxArg, max);

impl MaxArg<i8, i8> for field::I8Field {}
impl MaxArg<i16, i16> for field::I16Field {}
impl MaxArg<i32, i32> for field::I32Field {}
impl MaxArg<i64, i64> for field::I64Field {}
impl MaxArg<f32, f32> for field::F32Field {}
impl MaxArg<f64, f64> for field::F64Field {}
impl MaxArg<String, String> for field::StringField {}
impl MaxArg<time::Timespec, time::Timespec> for field::TimespecField {}

agg_func!(Sum, SumArg, sum);

impl SumArg<i64, i8> for field::I8Field {}
impl SumArg<i64, i16> for field::I16Field {}
impl SumArg<i64, i32> for field::I32Field {}
impl SumArg<i64, i64> for field::I64Field {}
impl SumArg<f64, f32> for field::F32Field {}
impl SumArg<f64, f64> for field::F64Field {}

agg_func!(Avg, AvgArg, avg);

impl AvgArg<i8, i8> for field::I8Field {}
impl AvgArg<i16, i16> for field::I16Field {}
impl AvgArg<i32, i32> for field::I32Field {}
impl AvgArg<i64, i64> for field::I64Field {}
impl AvgArg<f32, f32> for field::F32Field {}
impl AvgArg<f64, f64> for field::F64Field {}

agg_func!(Count, CountArg, count);

impl<T: 'static + expression::PrimitiveType + Clone> CountArg<i64, T> for field::NamedField<T> {}

#[derive(Clone, Copy, Debug)]
pub struct CountAll;

impl expression::UntypedExpression for CountAll {
    fn expression_as_sql(&self) -> &sql::ToSql {
        self
    }

    fn upcast_expression(&self) -> expression::SharedExpression {
        rc::Rc::new(Box::new(self.clone()) as expression::BoxedExpression)
    }
}

impl expression::Expression<i64> for CountAll { }
