
use std::sync::Arc;

use expression::{Expression, BoxedExpression, RcExpression, UntypedExpression};
use to_sql::{ToSql};
use field::{
    I8Field,
    I16Field,
    I32Field,
    I64Field,
    F32Field,
    F64Field,
};

pub trait SumArg<T: Clone>: Clone + Expression<T> + Send + Sync {
    fn sum(&self) -> Sum<T, Self> {
        Sum::new(self.clone())
    }
}

#[deriving(Clone)]
pub struct Sum<T, E: SumArg<T>> {
    pub expression: E
}

#[allow(dead_code)]
impl<T: Clone, E: SumArg<T>> Sum<T, E> {
    pub fn new(expr: E) -> Sum<T, E> {
        Sum {
            expression: expr.clone()
        }
    }
}

impl SumArg<i8> for I8Field {}
impl SumArg<i16> for I16Field {}
impl SumArg<i32> for I32Field {}
impl SumArg<i64> for I64Field {}
impl SumArg<f32> for F32Field {}
impl SumArg<f64> for F64Field {}

impl<T: Clone, E: SumArg<T>> UntypedExpression for Sum<T, E> {
    fn expression_as_sql(&self) -> &ToSql {
        self
    }

    fn upcast(&self) -> RcExpression {
        Arc::new(box self.clone() as BoxedExpression)
    }
}

impl<T: Clone, E: SumArg<T>> Expression<T> for Sum<T, E> { }