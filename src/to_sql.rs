use serialize::json::Json;
use time::Timespec;

use data_set::SelectDataSet;
use {Select, SelectOnly, SelectAll, From, NamedFrom, DataSetFrom};
use query::{RcQuery, IsQuery};
use field::{
    Field, 
    FieldDef, 

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

macro_rules! impl_for_is(
    ($field:ty, $value:ty, $formatter:expr) => (
        impl ToSql for IsQuery<$field, $value> {
            fn to_sql(&self) -> String {
                format!($formatter, self.field.name, self.value)
            }
        }   
    )
)

impl_for_is!(BoolField, bool, "{} = {}")
impl_for_is!(I8Field, i8, "{} = {}")
impl_for_is!(I16Field, i16, "{} = {}")
impl_for_is!(I32Field, i32, "{} = {}")
impl_for_is!(I64Field, i64, "{} = {}")
impl_for_is!(F32Field, f32, "{} = {}")
impl_for_is!(F64Field, f64, "{} = {}")
impl_for_is!(StringField, String, "{} = \"{}\"")
impl_for_is!(ByteListField, Vec<u8>, "{} = {}")
impl_for_is!(JsonField, Json, "{} = {}")
impl_for_is!(TimespecField, Timespec, "{} = {}")

impl ToSql for RcQuery {
    fn to_sql(&self) -> String {
        (***self).to_sql()
    }
}