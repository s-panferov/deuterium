use std::sync::Arc;
use serialize::json::Json;
use time::Timespec;

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