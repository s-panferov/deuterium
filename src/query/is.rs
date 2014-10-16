
use serialize::json::Json;
use time::Timespec;

use query::Query;
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
use to_sql::ToSql;

#[deriving(Send, Clone)]
pub struct IsQuery<F, T> {
    pub field: F,
    pub value: T
}

pub trait ToIsQuery<F, T> {
    fn is(&self, val: T) -> IsQuery<F, T>;
}

impl<T: Clone> ToIsQuery<NamedField<T>, T> for NamedField<T> {
    fn is(&self, val: T) -> IsQuery<NamedField<T>, T> {
        IsQuery {
            field: self.clone(),
            value: val
        }
    }
}

impl Query for IsQuery<BoolField, bool> { }
impl Query for IsQuery<I8Field, i8> { }
impl Query for IsQuery<I16Field, i16> { }
impl Query for IsQuery<I32Field, i32> { }
impl Query for IsQuery<I64Field, i64> { }
impl Query for IsQuery<F32Field, f32> { }
impl Query for IsQuery<F64Field, f64> { }
impl Query for IsQuery<StringField, String> { }
impl Query for IsQuery<ByteListField, Vec<u8>> { }
impl Query for IsQuery<JsonField, Json> { }
impl Query for IsQuery<TimespecField, Timespec> { }