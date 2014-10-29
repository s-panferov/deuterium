
use update_query::{
    UpdateQuery,
    FieldUpdate,
};

use sql::from::{FromToSql};
use sql::value::{ToPredicateValue};
use sql::{ToSql, QueryToSql};

impl<F: ToPredicateValue, T: ToPredicateValue> ToSql for FieldUpdate<F, T> {
    fn to_sql(&self) -> String {
        format!("{} = {}", self.get_field().to_predicate_value(), self.get_value().to_sql())
    }
}

impl<T, L, M> ToSql for UpdateQuery<T, L, M> {
    fn to_sql(&self) -> String {
        let mut sql = "UPDATE".to_string();

        if self.only {
            sql = format!("{} ONLY", sql)
        }

        sql = format!("{} {}", sql, self.table.to_from_sql());

        let updates_str: Vec<String> = self.updates.iter().map(|upd| upd.to_sql()).collect();
        sql = format!("{} SET {}", sql, updates_str.connect(", "));

        if self.from.is_some() {
            let from = self.from.as_ref().unwrap();
            if !from.is_empty() {
                let tables_str: Vec<String> = from.iter().map(|v| v.as_sql().to_from_sql()).collect();
                sql = format!("{} FROM {}", sql, tables_str.connect(", "))
            }
        }

        match self.where_.as_ref() {
            Some(predicate) => {
                sql = format!("{} WHERE {}", sql, predicate.to_sql(false))
            },
            None if !self.all => {
                // http://devopsreactions.tumblr.com/post/47352638154/almost-ran-update-without-where
                sql = format!("{} WHERE true = false", sql)
            },
            _ => ()
        }

        match &self.returning {
            &Some(ref select) => sql = format!("{} RETURNING {}", sql, select.to_sql()),
            &None => ()
        }

        sql
    }
}

impl<T, L, M> QueryToSql for UpdateQuery<T, L, M> {
    fn to_final_sql(&self) -> String {
        format!("{};", self.to_sql())
    }
}