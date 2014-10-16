use data_set::SelectDataSet;
use {Select, SelectOnly, SelectAll, From, NamedFrom, DataSetFrom};
use query::{RcQuery, IsQuery};
use field::{Field, NamedField, FieldDef};

pub trait ToSql {
    fn to_sql(&self) -> String;
}

impl ToSql for SelectDataSet {
    fn to_sql(&self) -> String {
        format!("SELECT {} FROM {} WHERE {};", 
            self.select.to_sql(), 
            self.from.to_sql(),
            self.where_.as_ref().unwrap().to_sql())
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

impl ToSql for IsQuery<NamedField<String>, String> {
    fn to_sql(&self) -> String {
        format!("{} = \"{}\"", self.field.name, self.value)
    }
}

impl ToSql for RcQuery {
    fn to_sql(&self) -> String {
        (***self).to_sql()
    }
}