
use serialize::json::Json;
use time::Timespec;

use query::{Query, RcQuery};
use expression::{RawExpression};

use field::{
    NamedField,

    BoolField, BoolComparable,
    I8Field, I8Comparable,
    I16Field, I16Comparable,
    I32Field, I32Comparable,
    I64Field, I64Comparable,
    F32Field, F32Comparable,
    F64Field, F64Comparable,
    StringField, StringComparable,
    ByteListField, ByteListComparable,
    JsonField, JsonComparable,
    TimespecField, TimespecComparable
};

#[deriving(Send, Clone)]
pub struct InQuery<F, T> {
    pub field: F,
    pub values: T
}

pub trait ToInQuery<F, T> {
    fn within(&self, val: T) -> RcQuery;
}

macro_rules! within_methods(
    ($v:ty) => (
        fn within(&self, values: $v) -> RcQuery {
            InQuery {
                field: self.clone(),
                values: values
            }.upcast()
        }
    )
)

macro_rules! impl_for(
    ($field:ty, $v:ident) => (
        impl<T: $v> Query for InQuery<$field, Vec<T>> { }

        impl<T: $v> ToInQuery<$field, Vec<T>> for $field {
            within_methods!(Vec<T>)   
        }
    )
)

impl_for!(I8Field, I8Comparable)
impl_for!(I16Field, I16Comparable)
impl_for!(I32Field, I32Comparable)
impl_for!(I64Field, I64Comparable)
impl_for!(F32Field, F32Comparable)
impl_for!(F64Field, F64Comparable)
impl_for!(StringField, StringComparable)
impl_for!(TimespecField, TimespecComparable)