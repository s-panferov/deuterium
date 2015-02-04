
use sql::{ToSql};
use std::rc::Rc;

use serialize::json::Json;
use time::Timespec;
use uuid::Uuid;
use std::mem;

use field::{
    BoolField,
    I8Field,
    I16Field,
    I32Field,
    I64Field,
    F32Field,
    F64Field,
    StringField,
    ByteListField,
    JsonField,
    TimespecField,
    UuidField,

    OptionalBoolField,
    OptionalI8Field,
    OptionalI16Field,
    OptionalI32Field,
    OptionalI64Field,
    OptionalF32Field,
    OptionalF64Field,
    OptionalStringField,
    OptionalByteListField,
    OptionalJsonField,
    OptionalTimespecField,
    OptionalUuidField,
};

pub trait Expression<T>: UntypedExpression {}
pub trait ListExpression<T>: UntypedExpression {}

pub trait UntypedExpression {
    fn expression_as_sql(&self) -> &ToSql;
    fn upcast_expression(&self) -> RcExpression;
}

pub type BoxedExpression = Box<UntypedExpression + 'static>;
pub type RcExpression = Rc<BoxedExpression>;

#[derive(Clone)]
pub enum ExprValue<T> {
    Value {
        expression: RcExpression
    },
    Default
}

pub trait ToExprValue<T> {
    fn to_expr_val(&self) -> ExprValue<T>;
}

impl<T> ExprValue<T> {
    pub fn new(exp: &Expression<T>) -> ExprValue<T> {
        ExprValue::Value {
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

        impl Expression<$t> for $t {
            
        }

        impl UntypedExpression for Vec<$t> {
            fn expression_as_sql(&self) -> &ToSql {
                self
            }

            fn upcast_expression(&self) -> RcExpression {
                Rc::new(Box::new(self.clone()) as BoxedExpression)
            }
        }

        impl ListExpression<$t> for Vec<$t> {
            
        }
    )
}

impl<'a, 'b, T> ToExprValue<T> for &'a (Expression<T> + 'b) {
    fn to_expr_val(&self) -> ExprValue<T> {
        ExprValue::new(*self)
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

pub trait ToExpression<T>: UntypedExpression {
    fn as_expr(&self) -> &Expression<T> { unsafe{ mem::transmute(self as &UntypedExpression) } }
}

pub trait ToListExpression<T>: UntypedExpression {
    fn as_expr(&self) -> &ListExpression<T> { unsafe{ mem::transmute(self as &UntypedExpression) } }
}

impl ToListExpression<bool> for Vec<bool> {}
impl ToListExpression<i8> for Vec<i8> {}
impl ToListExpression<i16> for Vec<i16> {}
impl ToListExpression<i32> for Vec<i32> {}
impl ToListExpression<i64> for Vec<i64> {}
impl ToListExpression<f32> for Vec<f32> {}
impl ToListExpression<f64> for Vec<f64> {}
impl ToListExpression<String> for Vec<String> {}
impl ToListExpression<Vec<u8>> for Vec<Vec<u8>> {}
impl ToListExpression<Json> for Vec<Json> {}
impl ToListExpression<Timespec> for Vec<Timespec> {}
impl ToListExpression<Uuid> for Vec<Uuid> {}

//
// Strings
//

impl ToExpression<String> for String {}
impl ToExpression<String> for StringField {}
impl ToExpression<String> for RawExpr {}

impl ToExpression<Option<String>> for String {}
impl ToExpression<Option<String>> for Option<String> {}
impl ToExpression<Option<String>> for StringField {}
impl ToExpression<Option<String>> for OptionalStringField {}
impl ToExpression<Option<String>> for RawExpr {}

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
        impl $comp for I8Field {} 
        impl $comp for I16Field {} 
        impl $comp for I32Field {} 
        impl $comp for I64Field {} 
        impl $comp for F32Field {} 
        impl $comp for F64Field {}         
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
        impl $comp for I8Field {} 
        impl $comp for I16Field {} 
        impl $comp for I32Field {} 
        impl $comp for I64Field {} 
        impl $comp for F32Field {} 
        impl $comp for F64Field {} 
        impl $comp for OptionalI8Field {} 
        impl $comp for OptionalI16Field {} 
        impl $comp for OptionalI32Field {} 
        impl $comp for OptionalI64Field {} 
        impl $comp for OptionalF32Field {} 
        impl $comp for OptionalF64Field {} 
        impl $comp for RawExpr {}
    )
}

cast_numbers!(ToExpression<i8>);
cast_numbers!(ToExpression<i16>);
cast_numbers!(ToExpression<i32>);
cast_numbers!(ToExpression<i64>);
cast_numbers!(ToExpression<f32>);
cast_numbers!(ToExpression<f64>);

cast_numbers_optional!(ToExpression<Option<i8>>);
cast_numbers_optional!(ToExpression<Option<i16>>);
cast_numbers_optional!(ToExpression<Option<i32>>);
cast_numbers_optional!(ToExpression<Option<i64>>);
cast_numbers_optional!(ToExpression<Option<f32>>);
cast_numbers_optional!(ToExpression<Option<f64>>);

//
// Boolean
//

impl ToExpression<bool> for bool {}
impl ToExpression<bool> for BoolField {} 
impl ToExpression<bool> for RawExpr {} 

impl ToExpression<Option<bool>> for bool {}
impl ToExpression<Option<bool>> for Option<bool> {}
impl ToExpression<Option<bool>> for BoolField {} 
impl ToExpression<Option<bool>> for OptionalBoolField {} 
impl ToExpression<Option<bool>> for RawExpr {} 

//
// Vec<u8>
//

impl ToExpression<Vec<u8>> for Vec<u8> {}
impl ToExpression<Vec<u8>> for ByteListField {}
impl ToExpression<Vec<u8>> for RawExpr {}

impl ToExpression<Option<Vec<u8>>> for Vec<u8> {}
impl ToExpression<Option<Vec<u8>>> for Option<Vec<u8>> {}
impl ToExpression<Option<Vec<u8>>> for ByteListField {}
impl ToExpression<Option<Vec<u8>>> for OptionalByteListField {}
impl ToExpression<Option<Vec<u8>>> for RawExpr {}

//
// Json
//

impl ToExpression<Json> for Json {}
impl ToExpression<Json> for JsonField {}
impl ToExpression<Json> for RawExpr {}

impl ToExpression<Option<Json>> for Json {}
impl ToExpression<Option<Json>> for Option<Json> {}
impl ToExpression<Option<Json>> for JsonField {}
impl ToExpression<Option<Json>> for OptionalJsonField {}
impl ToExpression<Option<Json>> for RawExpr {}

//
// Timespec
//

impl ToExpression<Timespec> for Timespec {}
impl ToExpression<Timespec> for TimespecField {}
impl ToExpression<Timespec> for RawExpr {}

impl ToExpression<Option<Timespec>> for Timespec {}
impl ToExpression<Option<Timespec>> for Option<Timespec> {}
impl ToExpression<Option<Timespec>> for TimespecField {}
impl ToExpression<Option<Timespec>> for OptionalTimespecField {}
impl ToExpression<Option<Timespec>> for RawExpr {}

//
// Uuid
//

impl ToExpression<Uuid> for Uuid {}
impl ToExpression<Uuid> for UuidField {}
impl ToExpression<Uuid> for RawExpr {}

impl ToExpression<Option<Uuid>> for Uuid {}
impl ToExpression<Option<Uuid>> for Option<Uuid> {}
impl ToExpression<Option<Uuid>> for UuidField {}
impl ToExpression<Option<Uuid>> for OptionalUuidField {}
impl ToExpression<Option<Uuid>> for RawExpr {}

//
// Untyped
//  

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
impl ToExpression<()> for Uuid {}
impl ToExpression<()> for Option<bool> {}
impl ToExpression<()> for Option<i8> {}
impl ToExpression<()> for Option<i16> {}
impl ToExpression<()> for Option<i32> {}
impl ToExpression<()> for Option<i64> {}
impl ToExpression<()> for Option<f32> {}
impl ToExpression<()> for Option<f64> {}
impl ToExpression<()> for Option<Vec<u8>> {}
impl ToExpression<()> for Option<String> {}
impl ToExpression<()> for Option<Json> {}
impl ToExpression<()> for Option<Timespec> {}
impl ToExpression<()> for Option<Uuid> {}
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
impl ToExpression<()> for UuidField {}
impl ToExpression<()> for OptionalBoolField {} 
impl ToExpression<()> for OptionalI8Field {} 
impl ToExpression<()> for OptionalI16Field {} 
impl ToExpression<()> for OptionalI32Field {} 
impl ToExpression<()> for OptionalI64Field {} 
impl ToExpression<()> for OptionalF32Field {} 
impl ToExpression<()> for OptionalF64Field {} 
impl ToExpression<()> for OptionalStringField {} 
impl ToExpression<()> for OptionalJsonField {} 
impl ToExpression<()> for OptionalByteListField {} 
impl ToExpression<()> for OptionalTimespecField {}
impl ToExpression<()> for OptionalUuidField {}
impl ToExpression<()> for RawExpr {}

impl ToExpression<RawExpr> for bool {}
impl ToExpression<RawExpr> for i8 {}
impl ToExpression<RawExpr> for i16 {}
impl ToExpression<RawExpr> for i32 {}
impl ToExpression<RawExpr> for i64 {}
impl ToExpression<RawExpr> for f32 {}
impl ToExpression<RawExpr> for f64 {}
impl ToExpression<RawExpr> for Vec<u8> {}
impl ToExpression<RawExpr> for String {}
impl ToExpression<RawExpr> for Json {}
impl ToExpression<RawExpr> for Timespec {}
impl ToExpression<RawExpr> for Uuid {}
impl ToExpression<RawExpr> for Option<bool> {}
impl ToExpression<RawExpr> for Option<i8> {}
impl ToExpression<RawExpr> for Option<i16> {}
impl ToExpression<RawExpr> for Option<i32> {}
impl ToExpression<RawExpr> for Option<i64> {}
impl ToExpression<RawExpr> for Option<f32> {}
impl ToExpression<RawExpr> for Option<f64> {}
impl ToExpression<RawExpr> for Option<Vec<u8>> {}
impl ToExpression<RawExpr> for Option<String> {}
impl ToExpression<RawExpr> for Option<Json> {}
impl ToExpression<RawExpr> for Option<Timespec> {}
impl ToExpression<RawExpr> for Option<Uuid> {}
impl ToExpression<RawExpr> for BoolField {} 
impl ToExpression<RawExpr> for I8Field {} 
impl ToExpression<RawExpr> for I16Field {} 
impl ToExpression<RawExpr> for I32Field {} 
impl ToExpression<RawExpr> for I64Field {} 
impl ToExpression<RawExpr> for F32Field {} 
impl ToExpression<RawExpr> for F64Field {} 
impl ToExpression<RawExpr> for StringField {} 
impl ToExpression<RawExpr> for JsonField {} 
impl ToExpression<RawExpr> for ByteListField {} 
impl ToExpression<RawExpr> for TimespecField {}
impl ToExpression<RawExpr> for UuidField {}
impl ToExpression<RawExpr> for OptionalBoolField {} 
impl ToExpression<RawExpr> for OptionalI8Field {} 
impl ToExpression<RawExpr> for OptionalI16Field {} 
impl ToExpression<RawExpr> for OptionalI32Field {} 
impl ToExpression<RawExpr> for OptionalI64Field {} 
impl ToExpression<RawExpr> for OptionalF32Field {} 
impl ToExpression<RawExpr> for OptionalF64Field {} 
impl ToExpression<RawExpr> for OptionalStringField {} 
impl ToExpression<RawExpr> for OptionalJsonField {} 
impl ToExpression<RawExpr> for OptionalByteListField {} 
impl ToExpression<RawExpr> for OptionalTimespecField {}
impl ToExpression<RawExpr> for OptionalUuidField {}