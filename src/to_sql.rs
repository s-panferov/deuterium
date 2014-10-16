use {DataSet, Select};

trait ToSql {
    fn to_sql(&self) -> String;
}

impl ToSql for DataSet {
    fn to_sql(&self) -> String {
        unimplemented!()
        // format!("SELECT {} WHERE {}", self.select.to_sql(), self.query.unwrap().to_sql())
    }
}

impl ToSql for Select {
    fn to_sql(&self) -> String {
        match self {
            &SelectOnly(ref names) => {
                names.connect(", ")
            },
            &SelectAll => "*".to_string()
        }
    }
}