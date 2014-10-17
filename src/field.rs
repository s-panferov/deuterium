use std::sync::Arc;
use serialize::json::Json;
use time::Timespec;

use to_sql::{ToQueryValue};
use expression::{RawExpression};

trait Coercer: Send {

}

pub trait Field {
    fn to_def(&self) -> FieldDef;
}

#[deriving(Clone)]
pub struct NamedField<T> {
    pub name: String
}

impl<T> Field for NamedField<T> {
    fn to_def(&self) -> FieldDef {
        FieldDef(self.name.to_string())
    }
}

#[deriving(Clone)]
pub struct FieldDef(String);

impl FieldDef {
    pub fn name(&self) -> &str {
        self.0.as_slice()
    }
}

pub type BoolField = NamedField<bool>;
pub type I8Field = NamedField<i8>;
pub type I16Field = NamedField<i16>;
pub type I32Field = NamedField<i32>;
pub type I64Field = NamedField<i64>;
pub type F32Field = NamedField<f32>;
pub type F64Field = NamedField<f64>;
pub type StringField = NamedField<String>;
pub type ByteListField = NamedField<Vec<u8>>;
pub type JsonField = NamedField<Json>;
pub type TimespecField = NamedField<Timespec>;

pub trait BoolComparable: Send + Clone + Sync + ToQueryValue { }
pub trait I8Comparable: Send + Clone + Sync + ToQueryValue { }
pub trait I16Comparable: Send + Clone + Sync + ToQueryValue { }
pub trait I32Comparable: Send + Clone + Sync + ToQueryValue { }
pub trait I64Comparable: Send + Clone + Sync + ToQueryValue { }
pub trait F32Comparable: Send + Clone + Sync + ToQueryValue { }
pub trait F64Comparable: Send + Clone + Sync + ToQueryValue { }
pub trait StringComparable: Send + Clone + Sync + ToQueryValue { }
pub trait ByteListComparable: Send + Clone + Sync + ToQueryValue { }
pub trait JsonComparable: Send + Clone + Sync + ToQueryValue { }
pub trait TimespecComparable: Send + Clone + Sync + ToQueryValue { }

impl BoolComparable for bool {}
impl BoolComparable for BoolField {} 
impl BoolComparable for RawExpression {} 

macro_rules! number_comparable(
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
        impl $comp for RawExpression {}
    )
)

number_comparable!(I8Comparable)
number_comparable!(I16Comparable)
number_comparable!(I32Comparable)
number_comparable!(I64Comparable)
number_comparable!(F32Comparable)
number_comparable!(F64Comparable)

impl StringComparable for String {}
impl StringComparable for StringField {}
impl StringComparable for RawExpression {}

impl ByteListComparable for Vec<u8> {}
impl ByteListComparable for ByteListField {}
impl ByteListComparable for RawExpression {}

impl JsonComparable for Json {}
impl JsonComparable for JsonField {}
impl JsonComparable for RawExpression {}

impl TimespecComparable for Timespec {}
impl TimespecComparable for TimespecField {}
impl TimespecComparable for RawExpression {}