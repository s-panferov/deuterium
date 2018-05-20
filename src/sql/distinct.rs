use super::super::distinct;

impl super::ToSql for distinct::Distinct {
    fn to_sql(&self, ctx: &mut super::SqlContext) -> String {
        match self.get_on() {
            &None => "DISTINCT".to_string(),
            &Some(ref on) if on.is_empty() => "DISTINCT".to_string(),
            &Some(ref on) => {
                let defs: Vec<String> = on.iter().map(|f| f.expression_as_sql().to_sql(ctx)).collect();
                format!("DISTINCT ON ({})", defs.join(", "))
            }
        }
    }
}
