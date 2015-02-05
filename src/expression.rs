
use sql::{ToSql, ToPredicateValue};
use std::rc::Rc;

use serialize::json::Json;
use time::Timespec;
use uuid::Uuid;
use std::mem;

use field;

#[derive(Clone)]
pub enum ExpressionValue<T> {
    Value {
        expression: RcExpression
    },
    Default
}

pub trait ToExpressionValue<T> {
    fn to_expr_val(&self) -> ExpressionValue<T>;
}

impl<T> ExpressionValue<T> {
    pub fn new(exp: &Expression<T>) -> ExpressionValue<T> {
        ExpressionValue::Value {
            expression: exp.upcast_expression()
        }
    }
}

#[derive(Clone)]
pub struct RawExpr {
    pub content: String
}

impl RawExpr {
    pub fn new(content: &str) -> RawExpr { 
        RawExpr {
            content: content.to_string()
        }
    }
}

pub trait UntypedExpression {
    fn expression_as_sql(&self) -> &ToSql;
    fn upcast_expression(&self) -> RcExpression;
}

pub trait Expression<T>: UntypedExpression {}
pub trait ListExpression<T>: UntypedExpression {}

pub type BoxedExpression = Box<UntypedExpression + 'static>;
pub type RcExpression = Rc<BoxedExpression>;

macro_rules! to_expression {
    ($t:ty) => (
        impl UntypedExpression for $t {
            fn expression_as_sql(&self) -> &ToSql {
                self
            }

            fn upcast_expression(&self) -> RcExpression {
                Rc::new(Box::new(self.clone()) as BoxedExpression)
            }
        }
    )
}

impl<'a, 'b, T> ToExpressionValue<T> for &'a (Expression<T> + 'b) {
    fn to_expr_val(&self) -> ExpressionValue<T> {
        ExpressionValue::new(*self)
    }   
}

to_expression!(bool);
to_expression!(i8);
to_expression!(i16);
to_expression!(i32);
to_expression!(i64);
to_expression!(f32);
to_expression!(f64);
to_expression!(String);
to_expression!(Vec<u8>);
to_expression!(Json);
to_expression!(Timespec);
to_expression!(Uuid);
to_expression!(RawExpr);

impl<T> Expression<T> for T where T: UntypedExpression { }
impl<T> Expression<T> for field::NamedField<T> where T: UntypedExpression + Clone { }
impl<T> Expression<Option<T>> for field::NamedField<T> where T: UntypedExpression + Clone { }

macro_rules! allow_cast {
    ($expr:ty, $($ty:ty),+) => (
        $(
            impl<T> $expr for T where T: Expression<$ty> { }
        )+
    )
}

macro_rules! allow_cast_for_optional {
    ($expr:ty, $($ty:ty),+) => (
        $(
            impl<T> $expr for T where T: Expression<Option<$ty>> { }
        )+
    )
}

allow_cast!(Expression<i8>, i16);
allow_cast!(Expression<i16>, i32);
allow_cast!(Expression<i32>, i64);
allow_cast!(Expression<i64>, f32);
allow_cast!(Expression<f32>, f64);

allow_cast_for_optional!(Expression<Option<i8>>, i16);
allow_cast_for_optional!(Expression<Option<i16>>, i32);
allow_cast_for_optional!(Expression<Option<i32>>, i64);
allow_cast_for_optional!(Expression<Option<i64>>, f32);
allow_cast_for_optional!(Expression<Option<f32>>, f64);

impl<T> UntypedExpression for Vec<T> where T: UntypedExpression + ToPredicateValue + Clone + 'static {
    fn expression_as_sql(&self) -> &ToSql {
        self
    }

    fn upcast_expression(&self) -> RcExpression {
        Rc::new(Box::new(self.clone()) as BoxedExpression)
    }
}

impl<T> ListExpression<Option<T>> for Vec<T> where T: UntypedExpression + ToPredicateValue + Clone + 'static {}
impl<T> ListExpression<T> for Vec<T> where T: UntypedExpression + ToPredicateValue + Clone + 'static {}