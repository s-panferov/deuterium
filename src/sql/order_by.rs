use super::super::order_by;

impl super::ToSql for order_by::OrderBy {
    fn to_sql(&self, ctx: &mut super::SqlContext) -> String {
        format!("{} {}", self.get_by().expression_as_sql().to_sql(ctx), match self.get_order() {
            &order_by::Order::Asc => "ASC",
            &order_by::Order::Desc => "DESC"
        })
    }
}
