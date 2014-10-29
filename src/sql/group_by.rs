use group_by::{GroupBy};
use sql::{ToSql};

impl ToSql for GroupBy {
    fn to_sql(&self) -> String {
        if !self.by.is_empty() {
            let defs: Vec<String> = self.by.iter().map(|f| f.expression_as_sql().to_sql()).collect();
            format!(" GROUP BY {}", defs.connect(", "))
        } else {
            String::new()
        }
    }
}
