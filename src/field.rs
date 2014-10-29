use serialize::json::Json;
use time::Timespec;
use std::sync::Arc;

use from::{Table};
use to_sql::{ToSql};
use expression::{Expression, UntypedExpression, RcExpression, BoxedExpression};

pub trait Field {
    fn name(&self) -> &str;
    fn table_name(&self) -> &str;
    fn qual(&self) -> Option<&String>;
    fn upcast_field(&self) -> RcField;
}

pub type BoxedField = Box<Field + Send + Sync>;
pub type RcField = Arc<BoxedField>;

#[deriving(Clone)]
pub struct NamedField<T> {
    pub name: String,
    pub table_name: String,
    pub qual: Option<String>
}

impl<T: Clone> NamedField<T> {
    pub fn new(name: &str, table_name: &str) -> NamedField<T>  {
        NamedField { 
            name: name.to_string(), 
            table_name: table_name.to_string(), 
            qual: None 
        }
    }

    pub fn new_qual(name: &str, table_name: &str, qual: &str) -> NamedField<T>  {
        NamedField { 
            name: name.to_string(), 
            table_name: table_name.to_string(),
            qual: Some(qual.to_string()) 
        }
    }

    pub fn field_of(name: &str, table: &Table) -> NamedField<T> {
        NamedField { 
            name: name.to_string(), 
            table_name: table.get_table_name().to_string(),
            qual: table.get_table_alias().as_ref().map(|v| v.to_string())
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

    pub fn qual_for(&self, table: &Table) -> NamedField<T> {
        let mut field = self.clone();
        field.qual = table.get_table_alias().as_ref().map(|v| v.to_string());
        field
    }
}

impl<T: Clone> UntypedExpression for NamedField<T> {
    fn expression_as_sql(&self) -> &ToSql {
        self
    }

    fn upcast_expression(&self) -> RcExpression {
        Arc::new(box self.clone() as BoxedExpression)
    }
}

impl<T: Clone> Expression<T> for NamedField<T> {}

impl<T: Clone> Field for NamedField<T> {
    fn name(&self) -> &str {
        self.name.as_slice()
    }

    fn table_name(&self) -> &str {
        self.table_name.as_slice()
    }

    fn qual(&self) -> Option<&String> {
        self.qual.as_ref()
    }

    fn upcast_field(&self) -> RcField {
        Arc::new(box self.clone() as BoxedField)
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