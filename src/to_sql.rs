use serialize::json::Json;
use time::Timespec;

use data_set::SelectDataSet;
use {Select, SelectOnly, SelectAll, From, NamedFrom, DataSetFrom};
use query::{RcQuery, IsQuery};
use field::{
    Field, 
    FieldDef,
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

pub trait ToSql {
    fn to_sql(&self) -> String;
}

impl ToSql for SelectDataSet {
    fn to_sql(&self) -> String {
        let mut sql = format!("SELECT {} FROM {}", 
            self.select.to_sql(), 
            self.from.to_sql()
        );
        
        if self.where_.is_some() {
            sql = format!("{} WHERE {}", sql, self.where_.as_ref().unwrap().to_sql())
        }

        format!("{};", sql)
    }
}

impl ToSql for From {
    fn to_sql(&self) -> String {
        match self {
            &NamedFrom(ref from) => {
                from.to_string()
            },
            &DataSetFrom(ref dset) => format!("( {} )", dset.to_sql())
        }
    }
}

impl ToSql for Select {
    fn to_sql(&self) -> String {
        match self {
            &SelectOnly(ref fields) => {
                let names: Vec<String> = fields.iter().map(|f| f.name().to_string()).collect();
                names.connect(", ")
            },
            &SelectAll => "*".to_string()
        }
    }
}

trait ToQueryValue {
    fn to_query_value(&self) -> String;
}

impl ToQueryValue for bool { fn to_query_value(&self) -> String { self.to_string() } }
impl ToQueryValue for i8 { fn to_query_value(&self) -> String { self.to_string() } }
impl ToQueryValue for i16 { fn to_query_value(&self) -> String { self.to_string() } }
impl ToQueryValue for i32 { fn to_query_value(&self) -> String { self.to_string() } }
impl ToQueryValue for i64 { fn to_query_value(&self) -> String { self.to_string() } }
impl ToQueryValue for f32 { fn to_query_value(&self) -> String { self.to_string() } }
impl ToQueryValue for f64 { fn to_query_value(&self) -> String { self.to_string() } }
impl ToQueryValue for String { 
    fn to_query_value(&self) -> String { format!("\"{}\"", self.to_string()) } 
}
impl ToQueryValue for Vec<u8> { fn to_query_value(&self) -> String { self.to_string() } }
impl ToQueryValue for Json { fn to_query_value(&self) -> String { self.to_string() } }
impl ToQueryValue for Timespec { fn to_query_value(&self) -> String { self.to_string() } }

impl<T: ToQueryValue> ToSql for IsQuery<NamedField<T>, T> {
    fn to_sql(&self) -> String {
        format!("{} = {}", self.field.name, self.value.to_query_value())
    }
}

impl ToSql for RcQuery {
    fn to_sql(&self) -> String {
        (***self).to_sql()
    }
}