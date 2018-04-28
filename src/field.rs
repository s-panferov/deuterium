use std::{fmt, rc};
use std::marker;
use serde_json;
use time;
use uuid;

use super::from;
use super::sql;
use super::expression;

pub trait Field {
    fn name(&self) -> &str;
    fn table_name(&self) -> &str;
    fn qual(&self) -> Option<&String>;
    fn upcast_field(&self) -> SharedField;
}

impl fmt::Debug for Field {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Field: {}", self.name())
    }
}

pub type BoxedField = Box<Field + 'static>;
pub type SharedField = rc::Rc<BoxedField>;

#[derive(Clone, Debug)]
pub struct NamedField<T> {
    pub name: String,
    pub table_name: String,
    pub qual: Option<String>,

    _marker: marker::PhantomData<T>,
}

impl<T: Clone> NamedField<T> {
    pub fn new(name: &str, table_name: &str) -> NamedField<T> {
        NamedField {
            name: name.to_string(),
            table_name: table_name.to_string(),
            qual: None,

            _marker: marker::PhantomData,
        }
    }

    pub fn new_qual(name: &str, table_name: &str, qual: &str) -> NamedField<T> {
        NamedField {
            name: name.to_string(),
            table_name: table_name.to_string(),
            qual: Some(qual.to_string()),

            _marker: marker::PhantomData,
        }
    }

    pub fn field_of(name: &str, table: &from::Table) -> NamedField<T> {
        NamedField {
            name: name.to_string(),
            table_name: table.get_table_name().to_string(),
            qual: table.get_table_alias().as_ref().map(|v| v.to_string()),

            _marker: marker::PhantomData,
        }
    }

    pub fn qual(&self) -> NamedField<T> {
        let mut field = self.clone();
        field.qual = Some(self.table_name.to_string());
        field
    }

    pub fn qual_with(&self, qual: &str) -> NamedField<T> {
        let mut field = self.clone();
        field.qual = Some(qual.to_string());
        field
    }

    pub fn qual_for(&self, table: &from::Table) -> NamedField<T> {
        let mut field = self.clone();
        field.qual = table.get_table_alias().as_ref().map(|v| v.to_string());
        field
    }
}

impl<T: Clone + 'static + fmt::Debug> expression::UntypedExpression for NamedField<T> {
    fn expression_as_sql(&self) -> &sql::ToSql {
        self
    }

    fn upcast_expression(&self) -> expression::SharedExpression {
        rc::Rc::new(Box::new(self.clone()) as expression::BoxedExpression)
    }
}

impl<T: Clone + 'static> Field for NamedField<T> {
    fn name(&self) -> &str {
        &self.name
    }

    fn table_name(&self) -> &str {
        &self.table_name
    }

    fn qual(&self) -> Option<&String> {
        self.qual.as_ref()
    }

    fn upcast_field(&self) -> SharedField {
        rc::Rc::new(Box::new(self.clone()))
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
pub type JsonField = NamedField<serde_json::Value>;
pub type TimespecField = NamedField<time::Timespec>;
pub type UuidField = NamedField<uuid::Uuid>;

pub type OptionalBoolField = NamedField<Option<bool>>;
pub type OptionalI8Field = NamedField<Option<i8>>;
pub type OptionalI16Field = NamedField<Option<i16>>;
pub type OptionalI32Field = NamedField<Option<i32>>;
pub type OptionalI64Field = NamedField<Option<i64>>;
pub type OptionalF32Field = NamedField<Option<f32>>;
pub type OptionalF64Field = NamedField<Option<f64>>;
pub type OptionalStringField = NamedField<Option<String>>;
pub type OptionalByteListField = NamedField<Option<Vec<u8>>>;
pub type OptionalJsonField = NamedField<Option<serde_json::Value>>;
pub type OptionalTimespecField = NamedField<Option<time::Timespec>>;
pub type OptionalUuidField = NamedField<Option<uuid::Uuid>>;
