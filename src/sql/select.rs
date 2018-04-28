use super::super::from;
use super::super::select_query;
use super::ToSql;
use std::fmt;

impl<T, L, M> super::from::FromToSql for from::FromSelect<T, L, M> {
    fn to_from_sql(&self, ctx: &mut super::SqlContext) -> String {
        format!("({}) as {}", self.select.to_sql(ctx), self.alias.to_string())
    }
}

impl ToSql for select_query::SelectFor {
    fn to_sql(&self, _ctx: &mut super::SqlContext) -> String {
        match self {
            &select_query::SelectFor::Update => "FOR UPDATE",
            &select_query::SelectFor::UpdateNoWait => "FOR UPDATE NOWAIT",
            &select_query::SelectFor::Share => "FOR SHARE",
            &select_query::SelectFor::ShareNoWait => "FOR SHARE NOWAIT",
        }.to_string()
    }
}

impl<T, L, M> ToSql for select_query::SelectQuery<T, L, M> {
    fn to_sql(&self, ctx: &mut super::SqlContext) -> String {
        let mut sql = "SELECT".to_string();

        if self.get_distinct().is_some() {
            sql = format!("{} {}", sql, self.get_distinct().as_ref().unwrap().to_sql(ctx));
        }

        sql = format!("{} {} FROM {}",
            sql,
            self.get_select().to_sql(ctx),
            self.get_from().as_sql().to_from_sql(ctx)
        );

        if !self.get_joins().is_empty() {
            let joins: Vec<String> = self.get_joins().iter().map(|join| join.to_sql(ctx)).collect();
            sql = format!("{} {}", sql, joins.join(" "))
        }

        if self.get_where().is_some() {
            sql = format!("{} WHERE {}", sql, self.get_where().as_ref().unwrap().to_sql(false, ctx));
        }

        if self.get_group_by().is_some() {
            sql = format!("{}{}", sql, self.get_group_by().as_ref().unwrap().to_sql(ctx));
        }

        if self.get_having().is_some() {
            sql = format!("{} HAVING {}", sql, self.get_having().as_ref().unwrap().to_sql(false, ctx));
        }

        if !self.get_order_by().is_empty() {
            let orders: Vec<String> = self.get_order_by().iter().map(|ord| ord.to_sql(ctx)).collect();
            sql = format!("{} ORDER BY {}", sql, orders.join(", "))
        }

        if self.get_limit().is_some() {
            sql = format!("{} LIMIT {}", sql, self.get_limit().unwrap())
        }

        if self.get_offset().is_some() {
            sql = format!("{} OFFSET {}", sql, self.get_offset().unwrap())
        }

        if self.get_for().is_some() {
            sql = format!("{} {}", sql, self.get_for().as_ref().unwrap().to_sql(ctx))
        }

        sql
    }
}

impl<T, L, M> super::QueryToSql for select_query::SelectQuery<T, L, M> {}

impl ToSql for select_query::SharedSelectQuery {
    fn to_sql(&self, ctx: &mut super::SqlContext) -> String {
        (**self).to_sql(ctx)
    }
}

impl ToSql for select_query::Select {
    fn to_sql(&self, ctx: &mut super::SqlContext) -> String {
        match self {
            &select_query::Select::Only(ref fields) => {
                let defs: Vec<String> = fields.iter().map(|f| f.expression_as_sql().to_sql(ctx)).collect();
                defs.join(", ")
            },
            &select_query::Select::All => "*".to_string()
        }
    }
}

impl<T: fmt::Debug, L: fmt::Debug, M: fmt::Debug> super::ToPredicateValue for select_query::SelectQuery<T, L, M> {
    fn to_predicate_value(&self, ctx: &mut super::SqlContext) -> String { self.to_sql(ctx) }
}
