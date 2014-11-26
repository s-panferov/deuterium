
use order_by::{OrderBy, Order};
use sql::{SqlContext, ToSql};

impl ToSql for OrderBy {
    fn to_sql(&self, ctx: &mut SqlContext) -> String {
        format!("{} {}", self.get_by().expression_as_sql().to_sql(ctx), match self.get_order() {
            &Order::Asc => "ASC",
            &Order::Desc => "DESC"
        })
    }
}
