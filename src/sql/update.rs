use super::super::update_query;
use super::from::{FromToSql};
use super::value;

impl<F: value::ToPredicateValue, T: value::ToPredicateValue> super::ToSql for update_query::FieldUpdate<F, T> {
    fn to_sql(&self, ctx: &mut super::SqlContext) -> String {
        format!("{} = {}", self.get_field().to_predicate_value(ctx), self.get_value().to_sql(ctx))
    }
}

impl<T, L, M> super::ToSql for update_query::UpdateQuery<T, L, M> {
    fn to_sql(&self, ctx: &mut super::SqlContext) -> String {
        let mut sql = "UPDATE".to_string();

        if self.is_only() {
            sql = format!("{} ONLY", sql)
        }

        sql = format!("{} {}", sql, self.get_table().to_from_sql(ctx));

        let updates_str: Vec<String> = self.get_updates().iter().map(|upd| upd.to_sql(ctx)).collect();
        sql = format!("{} SET {}", sql, updates_str.connect(", "));

        if self.get_from().is_some() {
            let from = self.get_from().as_ref().unwrap();
            if !from.is_empty() {
                let tables_str: Vec<String> = from.iter().map(|v| v.as_sql().to_from_sql(ctx)).collect();
                sql = format!("{} FROM {}", sql, tables_str.connect(", "))
            }
        }

        match self.get_where().as_ref() {
            Some(predicate) => {
                sql = format!("{} WHERE {}", sql, predicate.to_sql(false, ctx))
            },
            None if !self.is_all() => {
                // http://devopsreactions.tumblr.com/post/47352638154/almost-ran-update-without-where
                sql = format!("{} WHERE true = false", sql)
            },
            _ => ()
        }

        match self.get_returning() {
            &Some(ref select) => sql = format!("{} RETURNING {}", sql, select.to_sql(ctx)),
            &None => ()
        }

        sql
    }
}

impl<T, L, M> super::QueryToSql for update_query::UpdateQuery<T, L, M> {}