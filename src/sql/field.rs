use sql::{ToSql};
use field::{NamedField, RcField};

impl<T: Clone> ToSql for NamedField<T> {
    fn to_sql(&self) -> String {
        let ref name = self.name;
        match &self.qual {
            &Some(ref qual) => format!("{}.{}", qual, name),
            &None => name.to_string()
        }
    }
}

impl ToSql for RcField {
    fn to_sql(&self) -> String {
        let ref name = self.name();
        match &self.qual() {
            &Some(ref qual) => format!("{}.{}", qual, name),
            &None => name.to_string()
        }
    }
}