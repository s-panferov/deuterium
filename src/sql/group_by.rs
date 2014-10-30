use group_by::{GroupBy};
use sql::{SqlContext, ToSql};

impl ToSql for GroupBy {
    fn to_sql(&self, ctx: &mut SqlContext) -> String {
        if !self.by.is_empty() {
            let defs: Vec<String> = self.by.iter().map(|f| f.expression_as_sql().to_sql(ctx)).collect();
            format!(" GROUP BY {}", defs.connect(", "))
        } else {
            String::new()
        }
    }
}
