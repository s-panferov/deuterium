
use order_by::{OrderBy, Asc, Desc};
use sql::{ToSql};

impl ToSql for OrderBy {
    fn to_sql(&self) -> String {
        format!("{} {}", self.get_by().expression_as_sql().to_sql(), match self.get_order() {
            &Asc => "ASC",
            &Desc => "DESC"
        })
    }
}
