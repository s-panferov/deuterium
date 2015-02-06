use super::super::from;
use super::super::select_query;
use super::{ToSql};

impl<T, L, M> super::from::FromToSql for from::FromSelect<T, L, M> {
    fn to_from_sql(&self, ctx: &mut super::SqlContext) -> String {
        format!("({}) as {}", self.select.to_sql(ctx), self.alias.to_string())
    }
}

impl super::ToSql for select_query::SelectFor {
    fn to_sql(&self, _ctx: &mut super::SqlContext) -> String {
        match self {
            &select_query::SelectFor::Update => "FOR UPDATE",
            &select_query::SelectFor::UpdateNoWait => "FOR UPDATE NOWAIT",
            &select_query::SelectFor::Share => "FOR SHARE",
            &select_query::SelectFor::ShareNoWait => "FOR SHARE NOWAIT",
        }.to_string()
    }
}

impl<T, L, M> super::ToSql for select_query::SelectQuery<T, L, M> {
    fn to_sql(&self, ctx: &mut super::SqlContext) -> String {
        let mut sql = "SELECT".to_string();

        if self.distinct.is_some() {
            sql = format!("{} {}", sql, self.distinct.as_ref().unwrap().to_sql(ctx));
        }

        sql = format!("{} {} FROM {}", 
            sql,
            self.select.to_sql(ctx), 
            self.from.as_sql().to_from_sql(ctx)
        );

        if !self.joins.is_empty() {
            let joins: Vec<String> = self.joins.iter().map(|join| join.to_sql(ctx)).collect();
            sql = format!("{} {}", sql, joins.connect(" "))
        }

        if self.where_.is_some() {
            sql = format!("{} WHERE {}", sql, self.where_.as_ref().unwrap().to_sql(false, ctx));
        }

        if self.group_by.is_some() {
            sql = format!("{}{}", sql, self.group_by.as_ref().unwrap().to_sql(ctx));
        }

        if self.having.is_some() {
            sql = format!("{} HAVING {}", sql, self.having.as_ref().unwrap().to_sql(false, ctx));
        }

        if !self.order_by.is_empty() {
            let orders: Vec<String> = self.order_by.iter().map(|ord| ord.to_sql(ctx)).collect();
            sql = format!("{} ORDER BY {}", sql, orders.connect(", "))
        }

        if self.limit.is_some() {
            sql = format!("{} LIMIT {}", sql, self.limit.unwrap())
        }

        if self.offset.is_some() {
            sql = format!("{} OFFSET {}", sql, self.offset.unwrap())
        }

        if self.for_.is_some() {
            sql = format!("{} {}", sql, self.for_.as_ref().unwrap().to_sql(ctx))
        }

        sql
    }
}

impl<T, L, M> super::QueryToSql for select_query::SelectQuery<T, L, M> {}

impl super::ToSql for select_query::RcSelectQuery {
    fn to_sql(&self, ctx: &mut super::SqlContext) -> String {
        (**self).to_sql(ctx)
    }
}

impl super::ToSql for select_query::Select {
    fn to_sql(&self, ctx: &mut super::SqlContext) -> String {
        match self {
            &select_query::Select::Only(ref fields) => {
                let defs: Vec<String> = fields.iter().map(|f| f.expression_as_sql().to_sql(ctx)).collect();
                defs.connect(", ")
            },
            &select_query::Select::All => "*".to_string()
        }
    }
}

impl<T, L, M> super::ToPredicateValue for select_query::SelectQuery<T, L, M> {
    fn to_predicate_value(&self, ctx: &mut super::SqlContext) -> String { self.to_sql(ctx) }   
}