use super::super::group_by;

impl super::ToSql for group_by::GroupBy {
    fn to_sql(&self, ctx: &mut super::SqlContext) -> String {
        if !self.get_by().is_empty() {
            let defs: Vec<String> = self.get_by().iter().map(|f| f.expression_as_sql().to_sql(ctx)).collect();
            format!(" GROUP BY {}", defs.join(", "))
        } else {
            String::new()
        }
    }
}
