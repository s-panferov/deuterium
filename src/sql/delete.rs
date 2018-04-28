use super::super::delete_query;
use super::from::FromToSql;

impl<T, L, M> super::ToSql for delete_query::DeleteQuery<T, L, M> {
    fn to_sql(&self, ctx: &mut super::SqlContext) -> String {
        let mut sql = "DELETE FROM".to_string();

        if self.is_only() {
            sql = format!("{} ONLY", sql)
        }

        sql = format!("{} {}", sql, self.get_table().to_from_sql(ctx));

        if self.get_using().is_some() {
            let using = self.get_using().as_ref().unwrap();
            if !using.is_empty() {
                let tables_str: Vec<String> = using.iter().map(|v| v.as_sql().to_from_sql(ctx)).collect();
                sql = format!("{} USING {}", sql, tables_str.join(", "))
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

impl<T, L, M> super::QueryToSql for delete_query::DeleteQuery<T, L, M> {}
