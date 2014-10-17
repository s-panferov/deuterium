
use serialize::json::Json;
use time::Timespec;

use query::{Query, RcQuery};
use expression::{RawExpression};

use field::{
    NamedField,

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
    ($field:ty, $v:ty) => (
        impl Query for InQuery<$field, $v> { }

        impl ToInQuery<$field, $v> for $field {
            within_methods!($v)   
        }
    )
)

impl_for!(I8Field, Vec<i8>)
impl_for!(I16Field, Vec<i16>)
impl_for!(I32Field, Vec<i32>)
impl_for!(I64Field, Vec<i64>)
impl_for!(F32Field, Vec<f32>)
impl_for!(F64Field, Vec<f64>)
impl_for!(StringField, Vec<String>)
impl_for!(TimespecField, Vec<Timespec>)

impl Query for InQuery<RawExpression, Vec<RawExpression>> { }
impl ToInQuery<RawExpression, Vec<RawExpression>> for RawExpression {
    within_methods!(Vec<RawExpression>)   
}