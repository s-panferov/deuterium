use serialize::json::Json;
use time::Timespec;

use to_sql::{ToPredicateValue};
use expression::{RawExpression};

pub trait Field<T>: Send + Sync + Clone {
    fn to_def(&self) -> FieldDef<T>;
}

pub trait UntypedField: Send + Sync + Clone {
    fn to_def(&self) -> FieldDef<()>;
}

#[deriving(Clone)]
pub struct NamedField<T> {
    pub name: String
}

impl<T: Clone> Field<T> for NamedField<T> {
    fn to_def(&self) -> FieldDef<T> {
        FieldDef(self.name.to_string())
    }
}

impl<T: Clone> UntypedField for NamedField<T> {
    fn to_def(&self) -> FieldDef<()> {
        FieldDef(self.name.to_string())
    }
}

#[deriving(Clone)]
pub struct FieldDef<T>(String);

impl<T: Clone> FieldDef<T> {
    pub fn name(&self) -> String {
        self.0.to_string()
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

pub trait BoolComparable: Send + Clone + Sync + ToPredicateValue { }
pub trait I8Comparable: Send + Clone + Sync + ToPredicateValue { }
pub trait I16Comparable: Send + Clone + Sync + ToPredicateValue { }
pub trait I32Comparable: Send + Clone + Sync + ToPredicateValue { }
pub trait I64Comparable: Send + Clone + Sync + ToPredicateValue { }
pub trait F32Comparable: Send + Clone + Sync + ToPredicateValue { }
pub trait F64Comparable: Send + Clone + Sync + ToPredicateValue { }
pub trait StringComparable: Send + Clone + Sync + ToPredicateValue { }
pub trait ByteListComparable: Send + Clone + Sync + ToPredicateValue { }
pub trait JsonComparable: Send + Clone + Sync + ToPredicateValue { }
pub trait TimespecComparable: Send + Clone + Sync + ToPredicateValue { }

pub trait BoolComparableList: Send + Clone + Sync + ToPredicateValue { }
pub trait I8ComparableList: Send + Clone + Sync + ToPredicateValue{ }
pub trait I16ComparableList: Send + Clone + Sync + ToPredicateValue { }
pub trait I32ComparableList: Send + Clone + Sync + ToPredicateValue { }
pub trait I64ComparableList: Send + Clone + Sync + ToPredicateValue { }
pub trait F32ComparableList: Send + Clone + Sync + ToPredicateValue { }
pub trait F64ComparableList: Send + Clone + Sync + ToPredicateValue { }
pub trait StringComparableList: Send + Clone + Sync + ToPredicateValue { }
pub trait ByteListComparableList: Send + Clone + Sync + ToPredicateValue { }
pub trait JsonComparableList: Send + Clone + Sync + ToPredicateValue { }
pub trait TimespecComparableList: Send + Clone + Sync + ToPredicateValue { }

impl BoolComparableList for Vec<bool> {}
impl I8ComparableList for Vec<i8> {}
impl I16ComparableList for Vec<i16> {}
impl I32ComparableList for Vec<i32> {}
impl I64ComparableList for Vec<i64> {}
impl F32ComparableList for Vec<f32> {}
impl F64ComparableList for Vec<f64> {}
impl StringComparableList for Vec<String> {}
impl ByteListComparableList for Vec<Vec<u8>> {}
impl JsonComparableList for Vec<Json> {}
impl TimespecComparableList for Vec<Timespec> {}

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