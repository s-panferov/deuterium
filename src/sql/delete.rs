
use delete_query::{
    DeleteQuery,
};

use sql::from::{FromToSql};
use sql::{SqlContext, ToSql, QueryToSql};

impl<T, L, M> ToSql for DeleteQuery<T, L, M> {
    fn to_sql(&self, ctx: &mut SqlContext) -> String {
        let mut sql = "DELETE FROM".to_string();

        if self.only {
            sql = format!("{} ONLY", sql)
        }

        sql = format!("{} {}", sql, self.table.to_from_sql(ctx));

        if self.using.is_some() {
            let using = self.using.as_ref().unwrap();
            if !using.is_empty() {
                let tables_str: Vec<String> = using.iter().map(|v| v.as_sql().to_from_sql(ctx)).collect();
                sql = format!("{} USING {}", sql, tables_str.connect(", "))
            }
        }

        match self.where_.as_ref() {
            Some(predicate) => {
                sql = format!("{} WHERE {}", sql, predicate.to_sql(false, ctx))
            },
            None if !self.all => {
                // http://devopsreactions.tumblr.com/post/47352638154/almost-ran-update-without-where
                sql = format!("{} WHERE true = false", sql)
            },
            _ => ()
        }

        match &self.returning {
            &Some(ref select) => sql = format!("{} RETURNING {}", sql, select.to_sql(ctx)),
            &None => ()
        }

        sql
    }
}

impl<T, L, M> QueryToSql for DeleteQuery<T, L, M> {}
