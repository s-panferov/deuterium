use super::super::field;

impl<T: Clone> super::ToSql for field::NamedField<T> {
    fn to_sql(&self, _ctx: &mut super::SqlContext) -> String {
        let ref name = self.name;
        match &self.qual {
            &Some(ref qual) => format!("{}.{}", qual, name),
            &None => name.to_string()
        }
    }
}

impl super::ToSql for field::RcField {
    fn to_sql(&self, _ctx: &mut super::SqlContext) -> String {
        let ref name = self.name();
        match &self.qual() {
            &Some(ref qual) => format!("{}.{}", qual, name),
            &None => name.to_string()
        }
    }
}