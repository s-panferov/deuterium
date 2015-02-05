
use sql::{ToSql, ToPredicateValue};
use std::rc::Rc;

use serialize::json::Json;
use time::Timespec;
use uuid::Uuid;
use std::mem;

use field;

pub trait Expression<T>: UntypedExpression {}
pub trait ListExpression<T>: UntypedExpression {}

pub trait UntypedExpression {
    fn expression_as_sql(&self) -> &ToSql;
    fn upcast_expression(&self) -> RcExpression;
}

pub type BoxedExpression = Box<UntypedExpression + 'static>;
pub type RcExpression = Rc<BoxedExpression>;

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

macro_rules! impl_expression_for {
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

impl_expression_for!(bool);
impl_expression_for!(Option<bool>);
impl_expression_for!(i8);
impl_expression_for!(Option<i8>);
impl_expression_for!(i16);
impl_expression_for!(Option<i16>);
impl_expression_for!(i32);
impl_expression_for!(Option<i32>);
impl_expression_for!(i64);
impl_expression_for!(Option<i64>);
impl_expression_for!(f32);
impl_expression_for!(Option<f32>);
impl_expression_for!(f64);
impl_expression_for!(Option<f64>);
impl_expression_for!(String);
impl_expression_for!(Option<String>);
impl_expression_for!(Vec<u8>);
impl_expression_for!(Option<Vec<u8>>);
impl_expression_for!(Json);
impl_expression_for!(Option<Json>);
impl_expression_for!(Timespec);
impl_expression_for!(Option<Timespec>);
impl_expression_for!(Uuid);
impl_expression_for!(Option<Uuid>);
impl_expression_for!(RawExpr);
impl_expression_for!(Option<RawExpr>);

//
// Strings
//

impl Expression<String> for String {}
impl Expression<String> for field::StringField {}
impl Expression<String> for RawExpr {}

impl Expression<Option<String>> for String {}
impl Expression<Option<String>> for Option<String> {}
impl Expression<Option<String>> for field::StringField {}
impl Expression<Option<String>> for field::OptionalStringField {}
impl Expression<Option<String>> for RawExpr {}

//
// Numbers
//

macro_rules! cast_numbers {
    ($comp:ty) => (
        impl $comp for i8 {}
        impl $comp for i16 {}
        impl $comp for i32 {}
        impl $comp for i64 {}
        impl $comp for f32 {}
        impl $comp for f64 {}
        impl $comp for field::I8Field {} 
        impl $comp for field::I16Field {} 
        impl $comp for field::I32Field {} 
        impl $comp for field::I64Field {} 
        impl $comp for field::F32Field {} 
        impl $comp for field::F64Field {}         
        impl $comp for RawExpr {}
    )
}

macro_rules! cast_numbers_optional {
    ($comp:ty) => (
        impl $comp for i8 {}
        impl $comp for i16 {}
        impl $comp for i32 {}
        impl $comp for i64 {}
        impl $comp for f32 {}
        impl $comp for f64 {}
        impl $comp for field::I8Field {} 
        impl $comp for field::I16Field {} 
        impl $comp for field::I32Field {} 
        impl $comp for field::I64Field {} 
        impl $comp for field::F32Field {} 
        impl $comp for field::F64Field {} 
        impl $comp for field::OptionalI8Field {} 
        impl $comp for field::OptionalI16Field {} 
        impl $comp for field::OptionalI32Field {} 
        impl $comp for field::OptionalI64Field {} 
        impl $comp for field::OptionalF32Field {} 
        impl $comp for field::OptionalF64Field {} 
        impl $comp for RawExpr {}
    )
}

cast_numbers!(Expression<i8>);
cast_numbers!(Expression<i16>);
cast_numbers!(Expression<i32>);
cast_numbers!(Expression<i64>);
cast_numbers!(Expression<f32>);
cast_numbers!(Expression<f64>);

cast_numbers_optional!(Expression<Option<i8>>);
cast_numbers_optional!(Expression<Option<i16>>);
cast_numbers_optional!(Expression<Option<i32>>);
cast_numbers_optional!(Expression<Option<i64>>);
cast_numbers_optional!(Expression<Option<f32>>);
cast_numbers_optional!(Expression<Option<f64>>);

//
// Boolean
//

impl Expression<bool> for bool {}
impl Expression<bool> for field::BoolField {} 
impl Expression<bool> for RawExpr {} 

impl Expression<Option<bool>> for bool {}
impl Expression<Option<bool>> for Option<bool> {}
impl Expression<Option<bool>> for field::BoolField {} 
impl Expression<Option<bool>> for field::OptionalBoolField {} 
impl Expression<Option<bool>> for RawExpr {} 

//
// Vec<u8>
//

impl Expression<Vec<u8>> for Vec<u8> {}
impl Expression<Vec<u8>> for field::ByteListField {}
impl Expression<Vec<u8>> for RawExpr {}

impl Expression<Option<Vec<u8>>> for Vec<u8> {}
impl Expression<Option<Vec<u8>>> for Option<Vec<u8>> {}
impl Expression<Option<Vec<u8>>> for field::ByteListField {}
impl Expression<Option<Vec<u8>>> for field::OptionalByteListField {}
impl Expression<Option<Vec<u8>>> for RawExpr {}

//
// Json
//

impl Expression<Json> for Json {}
impl Expression<Json> for field::JsonField {}
impl Expression<Json> for RawExpr {}

impl Expression<Option<Json>> for Json {}
impl Expression<Option<Json>> for Option<Json> {}
impl Expression<Option<Json>> for field::JsonField {}
impl Expression<Option<Json>> for field::OptionalJsonField {}
impl Expression<Option<Json>> for RawExpr {}

//
// Timespec
//

impl Expression<Timespec> for Timespec {}
impl Expression<Timespec> for field::TimespecField {}
impl Expression<Timespec> for RawExpr {}

impl Expression<Option<Timespec>> for Timespec {}
impl Expression<Option<Timespec>> for Option<Timespec> {}
impl Expression<Option<Timespec>> for field::TimespecField {}
impl Expression<Option<Timespec>> for field::OptionalTimespecField {}
impl Expression<Option<Timespec>> for RawExpr {}

//
// Uuid
//

impl Expression<Uuid> for Uuid {}
impl Expression<Uuid> for field::UuidField {}
impl Expression<Uuid> for RawExpr {}

impl Expression<Option<Uuid>> for Uuid {}
impl Expression<Option<Uuid>> for Option<Uuid> {}
impl Expression<Option<Uuid>> for field::UuidField {}
impl Expression<Option<Uuid>> for field::OptionalUuidField {}
impl Expression<Option<Uuid>> for RawExpr {}

impl Expression<RawExpr> for bool {}
impl Expression<RawExpr> for i8 {}
impl Expression<RawExpr> for i16 {}
impl Expression<RawExpr> for i32 {}
impl Expression<RawExpr> for i64 {}
impl Expression<RawExpr> for f32 {}
impl Expression<RawExpr> for f64 {}
impl Expression<RawExpr> for Vec<u8> {}
impl Expression<RawExpr> for String {}
impl Expression<RawExpr> for Json {}
impl Expression<RawExpr> for Timespec {}
impl Expression<RawExpr> for Uuid {}
impl Expression<RawExpr> for Option<bool> {}
impl Expression<RawExpr> for Option<i8> {}
impl Expression<RawExpr> for Option<i16> {}
impl Expression<RawExpr> for Option<i32> {}
impl Expression<RawExpr> for Option<i64> {}
impl Expression<RawExpr> for Option<f32> {}
impl Expression<RawExpr> for Option<f64> {}
impl Expression<RawExpr> for Option<Vec<u8>> {}
impl Expression<RawExpr> for Option<String> {}
impl Expression<RawExpr> for Option<Json> {}
impl Expression<RawExpr> for Option<Timespec> {}
impl Expression<RawExpr> for Option<Uuid> {}
impl Expression<RawExpr> for field::BoolField {} 
impl Expression<RawExpr> for field::I8Field {} 
impl Expression<RawExpr> for field::I16Field {} 
impl Expression<RawExpr> for field::I32Field {} 
impl Expression<RawExpr> for field::I64Field {} 
impl Expression<RawExpr> for field::F32Field {} 
impl Expression<RawExpr> for field::F64Field {} 
impl Expression<RawExpr> for field::StringField {} 
impl Expression<RawExpr> for field::JsonField {} 
impl Expression<RawExpr> for field::ByteListField {} 
impl Expression<RawExpr> for field::TimespecField {}
impl Expression<RawExpr> for field::UuidField {}
impl Expression<RawExpr> for field::OptionalBoolField {} 
impl Expression<RawExpr> for field::OptionalI8Field {} 
impl Expression<RawExpr> for field::OptionalI16Field {} 
impl Expression<RawExpr> for field::OptionalI32Field {} 
impl Expression<RawExpr> for field::OptionalI64Field {} 
impl Expression<RawExpr> for field::OptionalF32Field {} 
impl Expression<RawExpr> for field::OptionalF64Field {} 
impl Expression<RawExpr> for field::OptionalStringField {} 
impl Expression<RawExpr> for field::OptionalJsonField {} 
impl Expression<RawExpr> for field::OptionalByteListField {} 
impl Expression<RawExpr> for field::OptionalTimespecField {}
impl Expression<RawExpr> for field::OptionalUuidField {}

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