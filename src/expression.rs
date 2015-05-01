use std::rc;
use std::mem;
use serialize::json;
use time;
use uuid;

use super::sql;
use super::field;

#[derive(Clone)]
/// Non-checking expression with any content you want.
pub struct RawExpression {
    pub content: String
}

impl RawExpression {
    pub fn new(content: &str) -> RawExpression {
        RawExpression {
            content: content.to_string()
        }
    }
}

/// Intrernal trait for all expressions. Allows some useful casts.
pub trait UntypedExpression {
    fn expression_as_sql(&self) -> &sql::ToSql;
    fn upcast_expression(&self) -> SharedExpression;
}

pub type BoxedExpression = Box<UntypedExpression + 'static>;
pub type SharedExpression = rc::Rc<BoxedExpression>;

/// Trait to indicate that value is an expression with concrete type.
pub trait Expression<T>: UntypedExpression {}

/// Trait to indicate that value is a LIST expression with concrete type.
pub trait ListExpression<T>: UntypedExpression {}

/// Trait to indicate that value is a primitive type that SQL adapter supports.
pub trait PrimitiveType { }

macro_rules! to_expression {
    ($t:ty) => (
        impl UntypedExpression for $t {
            fn expression_as_sql(&self) -> &sql::ToSql {
                self
            }

            fn upcast_expression(&self) -> SharedExpression {
                rc::Rc::new(Box::new(self.clone()))
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
to_expression!(RawExpression);

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
impl PrimitiveType for RawExpression { }

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
to_expression!(Option<RawExpression>);

// This trait is used to indicate that expression can be casted to
// expression of other type (e.g. numbers).
pub trait ToExpression<T>: UntypedExpression + Sized {
    fn as_expr(&self) -> &Expression<T> { unsafe{ mem::transmute(self as &UntypedExpression) } }
}

// This trait is used to indicate that expression can be casted to
// LIST expression of other type (e.g. list of numbers).
pub trait ToListExpression<T>: UntypedExpression + Sized {
    fn as_expr(&self) -> &ListExpression<T> { unsafe{ mem::transmute(self as &UntypedExpression) } }
}

impl<T> Expression<T>         for field::NamedField<T>         where T: PrimitiveType + Clone + 'static { }
impl<T> Expression<Option<T>> for field::NamedField<Option<T>> where T: PrimitiveType + Clone + 'static { }

//
// Strings
//

impl ToExpression<String> for String {}
impl ToExpression<String> for field::StringField {}
impl ToExpression<String> for RawExpression {}

impl ToExpression<Option<String>> for String {}
impl ToExpression<Option<String>> for Option<String> {}
impl ToExpression<Option<String>> for field::StringField {}
impl ToExpression<Option<String>> for field::OptionalStringField {}
impl ToExpression<Option<String>> for RawExpression {}

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
        impl $comp for RawExpression {}
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
        impl $comp for RawExpression {}
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
impl ToExpression<bool> for RawExpression {}

impl ToExpression<Option<bool>> for bool {}
impl ToExpression<Option<bool>> for Option<bool> {}
impl ToExpression<Option<bool>> for field::BoolField {}
impl ToExpression<Option<bool>> for field::OptionalBoolField {}
impl ToExpression<Option<bool>> for RawExpression {}

//
// Vec<u8>
//

impl ToExpression<Vec<u8>> for Vec<u8> {}
impl ToExpression<Vec<u8>> for field::ByteListField {}
impl ToExpression<Vec<u8>> for RawExpression {}

impl ToExpression<Option<Vec<u8>>> for Vec<u8> {}
impl ToExpression<Option<Vec<u8>>> for Option<Vec<u8>> {}
impl ToExpression<Option<Vec<u8>>> for field::ByteListField {}
impl ToExpression<Option<Vec<u8>>> for field::OptionalByteListField {}
impl ToExpression<Option<Vec<u8>>> for RawExpression {}

//
// json::Json
//

impl ToExpression<json::Json> for json::Json {}
impl ToExpression<json::Json> for field::JsonField {}
impl ToExpression<json::Json> for RawExpression {}

impl ToExpression<Option<json::Json>> for json::Json {}
impl ToExpression<Option<json::Json>> for Option<json::Json> {}
impl ToExpression<Option<json::Json>> for field::JsonField {}
impl ToExpression<Option<json::Json>> for field::OptionalJsonField {}
impl ToExpression<Option<json::Json>> for RawExpression {}

//
// time::Timespec
//

impl ToExpression<time::Timespec> for time::Timespec {}
impl ToExpression<time::Timespec> for field::TimespecField {}
impl ToExpression<time::Timespec> for RawExpression {}

impl ToExpression<Option<time::Timespec>> for time::Timespec {}
impl ToExpression<Option<time::Timespec>> for Option<time::Timespec> {}
impl ToExpression<Option<time::Timespec>> for field::TimespecField {}
impl ToExpression<Option<time::Timespec>> for field::OptionalTimespecField {}
impl ToExpression<Option<time::Timespec>> for RawExpression {}

//
// uuid::Uuid
//

impl ToExpression<uuid::Uuid> for uuid::Uuid {}
impl ToExpression<uuid::Uuid> for field::UuidField {}
impl ToExpression<uuid::Uuid> for RawExpression {}

impl ToExpression<Option<uuid::Uuid>> for uuid::Uuid {}
impl ToExpression<Option<uuid::Uuid>> for Option<uuid::Uuid> {}
impl ToExpression<Option<uuid::Uuid>> for field::UuidField {}
impl ToExpression<Option<uuid::Uuid>> for field::OptionalUuidField {}
impl ToExpression<Option<uuid::Uuid>> for RawExpression {}

impl ToExpression<RawExpression> for bool {}
impl ToExpression<RawExpression> for i8 {}
impl ToExpression<RawExpression> for i16 {}
impl ToExpression<RawExpression> for i32 {}
impl ToExpression<RawExpression> for i64 {}
impl ToExpression<RawExpression> for f32 {}
impl ToExpression<RawExpression> for f64 {}
impl ToExpression<RawExpression> for Vec<u8> {}
impl ToExpression<RawExpression> for String {}
impl ToExpression<RawExpression> for json::Json {}
impl ToExpression<RawExpression> for time::Timespec {}
impl ToExpression<RawExpression> for uuid::Uuid {}
impl ToExpression<RawExpression> for Option<bool> {}
impl ToExpression<RawExpression> for Option<i8> {}
impl ToExpression<RawExpression> for Option<i16> {}
impl ToExpression<RawExpression> for Option<i32> {}
impl ToExpression<RawExpression> for Option<i64> {}
impl ToExpression<RawExpression> for Option<f32> {}
impl ToExpression<RawExpression> for Option<f64> {}
impl ToExpression<RawExpression> for Option<Vec<u8>> {}
impl ToExpression<RawExpression> for Option<String> {}
impl ToExpression<RawExpression> for Option<json::Json> {}
impl ToExpression<RawExpression> for Option<time::Timespec> {}
impl ToExpression<RawExpression> for Option<uuid::Uuid> {}
impl ToExpression<RawExpression> for field::BoolField {}
impl ToExpression<RawExpression> for field::I8Field {}
impl ToExpression<RawExpression> for field::I16Field {}
impl ToExpression<RawExpression> for field::I32Field {}
impl ToExpression<RawExpression> for field::I64Field {}
impl ToExpression<RawExpression> for field::F32Field {}
impl ToExpression<RawExpression> for field::F64Field {}
impl ToExpression<RawExpression> for field::StringField {}
impl ToExpression<RawExpression> for field::JsonField {}
impl ToExpression<RawExpression> for field::ByteListField {}
impl ToExpression<RawExpression> for field::TimespecField {}
impl ToExpression<RawExpression> for field::UuidField {}
impl ToExpression<RawExpression> for field::OptionalBoolField {}
impl ToExpression<RawExpression> for field::OptionalI8Field {}
impl ToExpression<RawExpression> for field::OptionalI16Field {}
impl ToExpression<RawExpression> for field::OptionalI32Field {}
impl ToExpression<RawExpression> for field::OptionalI64Field {}
impl ToExpression<RawExpression> for field::OptionalF32Field {}
impl ToExpression<RawExpression> for field::OptionalF64Field {}
impl ToExpression<RawExpression> for field::OptionalStringField {}
impl ToExpression<RawExpression> for field::OptionalJsonField {}
impl ToExpression<RawExpression> for field::OptionalByteListField {}
impl ToExpression<RawExpression> for field::OptionalTimespecField {}
impl ToExpression<RawExpression> for field::OptionalUuidField {}

impl<T> UntypedExpression for Vec<T> where T: UntypedExpression + sql::ToPredicateValue + Clone + 'static {
    fn expression_as_sql(&self) -> &sql::ToSql {
        self
    }

    fn upcast_expression(&self) -> SharedExpression {
        rc::Rc::new(Box::new(self.clone()))
    }
}

impl<T> ListExpression<T> for Vec<T> where T: UntypedExpression + sql::ToPredicateValue + Clone + 'static {}
impl<T> ListExpression<Option<T>> for Vec<T> where T: UntypedExpression + sql::ToPredicateValue + Clone + 'static {}
impl<T> ToListExpression<T> for Vec<T> where T: UntypedExpression + sql::ToPredicateValue + Clone + 'static { }
