use std::rc;
use std::mem;
use serialize::json;
use time;
use uuid;

use super::sql;
use super::field;

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
    fn expression_as_sql(&self) -> &sql::ToSql;
    fn upcast_expression(&self) -> RcExpression;
}

pub trait Expression<T>: UntypedExpression {}
pub trait ListExpression<T>: UntypedExpression {}

pub type BoxedExpression = Box<UntypedExpression + 'static>;
pub type RcExpression = rc::Rc<BoxedExpression>;

pub trait PrimitiveType { }

macro_rules! to_expression {
    ($t:ty) => (
        impl UntypedExpression for $t {
            fn expression_as_sql(&self) -> &sql::ToSql {
                self
            }

            fn upcast_expression(&self) -> RcExpression {
                rc::Rc::new(Box::new(self.clone()) as BoxedExpression)
            }
        }
        
        impl Expression<$t> for $t { }
    )
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
to_expression!(json::Json);
to_expression!(time::Timespec);
to_expression!(uuid::Uuid);
to_expression!(RawExpr);

impl PrimitiveType for bool { }
impl PrimitiveType for i8 { }
impl PrimitiveType for i16 { }
impl PrimitiveType for i32 { }
impl PrimitiveType for i64 { }
impl PrimitiveType for f32 { }
impl PrimitiveType for f64 { }
impl PrimitiveType for String { }
impl PrimitiveType for Vec<u8> { }
impl PrimitiveType for json::Json { }
impl PrimitiveType for time::Timespec { }
impl PrimitiveType for uuid::Uuid { }
impl PrimitiveType for RawExpr { }

to_expression!(Option<bool>);
to_expression!(Option<i8>);
to_expression!(Option<i16>);
to_expression!(Option<i32>);
to_expression!(Option<i64>);
to_expression!(Option<f32>);
to_expression!(Option<f64>);
to_expression!(Option<String>);
to_expression!(Option<Vec<u8>>);
to_expression!(Option<json::Json>);
to_expression!(Option<time::Timespec>);
to_expression!(Option<uuid::Uuid>);
to_expression!(Option<RawExpr>);

pub trait ToExpression<T>: UntypedExpression + Sized {
    fn as_expr(&self) -> &Expression<T> { unsafe{ mem::transmute(self as &UntypedExpression) } }
}

pub trait ToListExpression<T>: UntypedExpression + Sized {
    fn as_expr(&self) -> &ListExpression<T> { unsafe{ mem::transmute(self as &UntypedExpression) } }
}

impl<T> Expression<T>         for field::NamedField<T>         where T: PrimitiveType + Clone { }
impl<T> Expression<Option<T>> for field::NamedField<Option<T>> where T: PrimitiveType + Clone { }

//
// Strings
//

impl ToExpression<String> for String {}
impl ToExpression<String> for field::StringField {}
impl ToExpression<String> for RawExpr {}

impl ToExpression<Option<String>> for String {}
impl ToExpression<Option<String>> for Option<String> {}
impl ToExpression<Option<String>> for field::StringField {}
impl ToExpression<Option<String>> for field::OptionalStringField {}
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
impl ToExpression<bool> for field::BoolField {} 
impl ToExpression<bool> for RawExpr {} 

impl ToExpression<Option<bool>> for bool {}
impl ToExpression<Option<bool>> for Option<bool> {}
impl ToExpression<Option<bool>> for field::BoolField {} 
impl ToExpression<Option<bool>> for field::OptionalBoolField {} 
impl ToExpression<Option<bool>> for RawExpr {} 

//
// Vec<u8>
//

impl ToExpression<Vec<u8>> for Vec<u8> {}
impl ToExpression<Vec<u8>> for field::ByteListField {}
impl ToExpression<Vec<u8>> for RawExpr {}

impl ToExpression<Option<Vec<u8>>> for Vec<u8> {}
impl ToExpression<Option<Vec<u8>>> for Option<Vec<u8>> {}
impl ToExpression<Option<Vec<u8>>> for field::ByteListField {}
impl ToExpression<Option<Vec<u8>>> for field::OptionalByteListField {}
impl ToExpression<Option<Vec<u8>>> for RawExpr {}

//
// json::Json
//

impl ToExpression<json::Json> for json::Json {}
impl ToExpression<json::Json> for field::JsonField {}
impl ToExpression<json::Json> for RawExpr {}

impl ToExpression<Option<json::Json>> for json::Json {}
impl ToExpression<Option<json::Json>> for Option<json::Json> {}
impl ToExpression<Option<json::Json>> for field::JsonField {}
impl ToExpression<Option<json::Json>> for field::OptionalJsonField {}
impl ToExpression<Option<json::Json>> for RawExpr {}

//
// time::Timespec
//

impl ToExpression<time::Timespec> for time::Timespec {}
impl ToExpression<time::Timespec> for field::TimespecField {}
impl ToExpression<time::Timespec> for RawExpr {}

impl ToExpression<Option<time::Timespec>> for time::Timespec {}
impl ToExpression<Option<time::Timespec>> for Option<time::Timespec> {}
impl ToExpression<Option<time::Timespec>> for field::TimespecField {}
impl ToExpression<Option<time::Timespec>> for field::OptionalTimespecField {}
impl ToExpression<Option<time::Timespec>> for RawExpr {}

//
// uuid::Uuid
//

impl ToExpression<uuid::Uuid> for uuid::Uuid {}
impl ToExpression<uuid::Uuid> for field::UuidField {}
impl ToExpression<uuid::Uuid> for RawExpr {}

impl ToExpression<Option<uuid::Uuid>> for uuid::Uuid {}
impl ToExpression<Option<uuid::Uuid>> for Option<uuid::Uuid> {}
impl ToExpression<Option<uuid::Uuid>> for field::UuidField {}
impl ToExpression<Option<uuid::Uuid>> for field::OptionalUuidField {}
impl ToExpression<Option<uuid::Uuid>> for RawExpr {}

impl ToExpression<()> for bool {}
impl ToExpression<()> for i8 {}
impl ToExpression<()> for i16 {}
impl ToExpression<()> for i32 {}
impl ToExpression<()> for i64 {}
impl ToExpression<()> for f32 {}
impl ToExpression<()> for f64 {}
impl ToExpression<()> for Vec<u8> {}
impl ToExpression<()> for String {}
impl ToExpression<()> for json::Json {}
impl ToExpression<()> for time::Timespec {}
impl ToExpression<()> for uuid::Uuid {}
impl ToExpression<()> for Option<bool> {}
impl ToExpression<()> for Option<i8> {}
impl ToExpression<()> for Option<i16> {}
impl ToExpression<()> for Option<i32> {}
impl ToExpression<()> for Option<i64> {}
impl ToExpression<()> for Option<f32> {}
impl ToExpression<()> for Option<f64> {}
impl ToExpression<()> for Option<Vec<u8>> {}
impl ToExpression<()> for Option<String> {}
impl ToExpression<()> for Option<json::Json> {}
impl ToExpression<()> for Option<time::Timespec> {}
impl ToExpression<()> for Option<uuid::Uuid> {}
impl ToExpression<()> for field::BoolField {} 
impl ToExpression<()> for field::I8Field {} 
impl ToExpression<()> for field::I16Field {} 
impl ToExpression<()> for field::I32Field {} 
impl ToExpression<()> for field::I64Field {} 
impl ToExpression<()> for field::F32Field {} 
impl ToExpression<()> for field::F64Field {} 
impl ToExpression<()> for field::StringField {} 
impl ToExpression<()> for field::JsonField {} 
impl ToExpression<()> for field::ByteListField {} 
impl ToExpression<()> for field::TimespecField {}
impl ToExpression<()> for field::UuidField {}
impl ToExpression<()> for field::OptionalBoolField {} 
impl ToExpression<()> for field::OptionalI8Field {} 
impl ToExpression<()> for field::OptionalI16Field {} 
impl ToExpression<()> for field::OptionalI32Field {} 
impl ToExpression<()> for field::OptionalI64Field {} 
impl ToExpression<()> for field::OptionalF32Field {} 
impl ToExpression<()> for field::OptionalF64Field {} 
impl ToExpression<()> for field::OptionalStringField {} 
impl ToExpression<()> for field::OptionalJsonField {} 
impl ToExpression<()> for field::OptionalByteListField {} 
impl ToExpression<()> for field::OptionalTimespecField {}
impl ToExpression<()> for field::OptionalUuidField {}

impl ToExpression<RawExpr> for bool {}
impl ToExpression<RawExpr> for i8 {}
impl ToExpression<RawExpr> for i16 {}
impl ToExpression<RawExpr> for i32 {}
impl ToExpression<RawExpr> for i64 {}
impl ToExpression<RawExpr> for f32 {}
impl ToExpression<RawExpr> for f64 {}
impl ToExpression<RawExpr> for Vec<u8> {}
impl ToExpression<RawExpr> for String {}
impl ToExpression<RawExpr> for json::Json {}
impl ToExpression<RawExpr> for time::Timespec {}
impl ToExpression<RawExpr> for uuid::Uuid {}
impl ToExpression<RawExpr> for Option<bool> {}
impl ToExpression<RawExpr> for Option<i8> {}
impl ToExpression<RawExpr> for Option<i16> {}
impl ToExpression<RawExpr> for Option<i32> {}
impl ToExpression<RawExpr> for Option<i64> {}
impl ToExpression<RawExpr> for Option<f32> {}
impl ToExpression<RawExpr> for Option<f64> {}
impl ToExpression<RawExpr> for Option<Vec<u8>> {}
impl ToExpression<RawExpr> for Option<String> {}
impl ToExpression<RawExpr> for Option<json::Json> {}
impl ToExpression<RawExpr> for Option<time::Timespec> {}
impl ToExpression<RawExpr> for Option<uuid::Uuid> {}
impl ToExpression<RawExpr> for field::BoolField {} 
impl ToExpression<RawExpr> for field::I8Field {} 
impl ToExpression<RawExpr> for field::I16Field {} 
impl ToExpression<RawExpr> for field::I32Field {} 
impl ToExpression<RawExpr> for field::I64Field {} 
impl ToExpression<RawExpr> for field::F32Field {} 
impl ToExpression<RawExpr> for field::F64Field {} 
impl ToExpression<RawExpr> for field::StringField {} 
impl ToExpression<RawExpr> for field::JsonField {} 
impl ToExpression<RawExpr> for field::ByteListField {} 
impl ToExpression<RawExpr> for field::TimespecField {}
impl ToExpression<RawExpr> for field::UuidField {}
impl ToExpression<RawExpr> for field::OptionalBoolField {} 
impl ToExpression<RawExpr> for field::OptionalI8Field {} 
impl ToExpression<RawExpr> for field::OptionalI16Field {} 
impl ToExpression<RawExpr> for field::OptionalI32Field {} 
impl ToExpression<RawExpr> for field::OptionalI64Field {} 
impl ToExpression<RawExpr> for field::OptionalF32Field {} 
impl ToExpression<RawExpr> for field::OptionalF64Field {} 
impl ToExpression<RawExpr> for field::OptionalStringField {} 
impl ToExpression<RawExpr> for field::OptionalJsonField {} 
impl ToExpression<RawExpr> for field::OptionalByteListField {} 
impl ToExpression<RawExpr> for field::OptionalTimespecField {}
impl ToExpression<RawExpr> for field::OptionalUuidField {}

impl<T> UntypedExpression for Vec<T> where T: UntypedExpression + sql::ToPredicateValue + Clone + 'static {
    fn expression_as_sql(&self) -> &sql::ToSql {
        self
    }

    fn upcast_expression(&self) -> RcExpression {
        rc::Rc::new(Box::new(self.clone()) as BoxedExpression)
    }
}

impl<T> ListExpression<T> for Vec<T> where T: UntypedExpression + sql::ToPredicateValue + Clone + 'static {}
impl<T> ListExpression<Option<T>> for Vec<T> where T: UntypedExpression + sql::ToPredicateValue + Clone + 'static {}
impl<T> ToListExpression<T> for Vec<T> where T: UntypedExpression + sql::ToPredicateValue + Clone + 'static { }