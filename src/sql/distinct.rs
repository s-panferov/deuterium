
use distinct::{Distinct};
use sql::{ToSql};

impl ToSql for Distinct {
    fn to_sql(&self) -> String {
        match &self.on {
            &None => "DISTINCT".to_string(),
            &Some(ref on) if on.is_empty() => "DISTINCT".to_string(),
            &Some(ref on) => {
                let defs: Vec<String> = on.iter().map(|f| f.expression_as_sql().to_sql()).collect();
                format!("DISTINCT ON ({})", defs.connect(", "))
            }
        }
    }
}
