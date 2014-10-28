
use to_sql::{ToSql};
use std::sync::Arc;

use serialize::json::Json;
use time::Timespec;
use std::mem;

use field::{
    StringField,
    BoolField,
    ByteListField,
    JsonField,
    TimespecField,
    I8Field,
    I16Field,
    I32Field,
    I64Field,
    F32Field,
    F64Field,
};

pub trait Expression<T> for Sized?: UntypedExpression {}

pub trait UntypedExpression for Sized? {
    fn expression_as_sql(&self) -> &ToSql;
    fn upcast_expression(&self) -> RcExpression;
}

pub type BoxedExpression = Box<UntypedExpression + Send + Sync>;
pub type RcExpression = Arc<BoxedExpression>;

#[deriving(Clone)]
pub enum ExprValue<T> {
    ExpressionValue {
        expression: RcExpression
    },
    DefaultValue
}

pub trait ToExprValue<T> for Sized? {
    fn to_expr_val(&self) -> ExprValue<T>;
}

impl<T> ExprValue<T> {
    pub fn new(exp: &Expression<T>) -> ExprValue<T> {
        ExpressionValue {
            expression: exp.upcast_expression()
        }
    }
}

impl<T: Clone> ToExprValue<()> for ExprValue<T> {
    fn to_expr_val(&self) -> ExprValue<()> {
        unsafe {
            mem::transmute(self.clone())
        }
    }
}

macro_rules! impl_expression_for(
    ($t:ty) => (
        impl UntypedExpression for $t {
            fn expression_as_sql(&self) -> &ToSql {
                self
            }

            fn upcast_expression(&self) -> RcExpression {
                Arc::new(box self.clone() as BoxedExpression)
            }
        }

        impl Expression<$t> for $t {
            
        }

        impl ToExprValue<$t> for $t {
            fn to_expr_val(&self) -> ExprValue<$t> {
                ExprValue::new(self)
            }
        }

        impl ToExprValue<()> for $t {
            fn to_expr_val(&self) -> ExprValue<()> {
                ExprValue::new(self).to_expr_val()
            }
        }
    )
)

impl_expression_for!(bool)
impl_expression_for!(i8)
impl_expression_for!(i16)
impl_expression_for!(i32)
impl_expression_for!(i64)
impl_expression_for!(f32)
impl_expression_for!(f64)
impl_expression_for!(String)
impl_expression_for!(Vec<u8>)
impl_expression_for!(Json)
impl_expression_for!(Timespec)

pub trait ToExpression<T>: UntypedExpression {
    fn as_expr(&self) -> &Expression<T> { unsafe{ mem::transmute(self as &UntypedExpression) } }
}

macro_rules! cast_numbers(
    ($comp:ty) => (
        impl $comp for i8 {}
        impl $comp for i16 {}
        impl $comp for i32 {}
        impl $comp for i64 {}
        impl $comp for f32 {}
        impl $comp for f64 {}
        impl $comp for I8Field {} 
        impl $comp for I16Field {} 
        impl $comp for I32Field {} 
        impl $comp for I64Field {} 
        impl $comp for F32Field {} 
        impl $comp for F64Field {} 
    )
)

impl ToExpression<String> for String {}
impl ToExpression<String> for StringField {}

cast_numbers!(ToExpression<i8>)
cast_numbers!(ToExpression<i16>)
cast_numbers!(ToExpression<i32>)
cast_numbers!(ToExpression<i64>)
cast_numbers!(ToExpression<f32>)
cast_numbers!(ToExpression<f64>)

impl ToExpression<bool> for bool {}
impl ToExpression<bool> for BoolField {} 

impl ToExpression<Vec<u8>> for Vec<u8> {}
impl ToExpression<Vec<u8>> for ByteListField {}

impl ToExpression<Json> for Json {}
impl ToExpression<Json> for JsonField {}

impl ToExpression<Timespec> for Timespec {}
impl ToExpression<Timespec> for TimespecField {}

impl ToExpression<()> for bool {}
impl ToExpression<()> for i8 {}
impl ToExpression<()> for i16 {}
impl ToExpression<()> for i32 {}
impl ToExpression<()> for i64 {}
impl ToExpression<()> for f32 {}
impl ToExpression<()> for f64 {}
impl ToExpression<()> for Vec<u8> {}
impl ToExpression<()> for String {}
impl ToExpression<()> for Json {}
impl ToExpression<()> for Timespec {}
impl ToExpression<()> for BoolField {} 
impl ToExpression<()> for I8Field {} 
impl ToExpression<()> for I16Field {} 
impl ToExpression<()> for I32Field {} 
impl ToExpression<()> for I64Field {} 
impl ToExpression<()> for F32Field {} 
impl ToExpression<()> for F64Field {} 
impl ToExpression<()> for StringField {} 
impl ToExpression<()> for JsonField {} 
impl ToExpression<()> for ByteListField {} 
impl ToExpression<()> for TimespecField {} 