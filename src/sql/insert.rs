use super::super::insert_query;

impl<T: Clone, V: super::ToSql, M: Clone> super::ToSql for insert_query::Insert<T, V, M> {
    fn to_sql(&self, ctx: &mut super::SqlContext) -> String {
        match self {
            &insert_query::Insert::DefaultValues => {
                format!("DEFAULT VALUES")
            },
            &insert_query::Insert::Values(ref rows) => {
                let rows_str: Vec<String> = rows.iter().map(|row| { format!("({})", row.to_sql(ctx)) }).collect();
                format!("VALUES\n    {}", rows_str.connect(",\n    "))
            },
            &insert_query::Insert::UntypedValues(ref rows) => {
                let rows_str: Vec<String> = rows.iter().map(|row| {
                    let values_str: Vec<String> = row.iter().map(|v| v.to_sql(ctx)).collect();
                    format!("({})", values_str.connect(", "))
                }).collect();
                format!("VALUES\n    {}", rows_str.connect(",\n    "))
            },
            &insert_query::Insert::FromSelect(ref select) => {
                select.to_sql(ctx)
            }
        }
    }
}

impl<T: Clone, V: Clone+super::ToSql, M: Clone, RT: Clone, RL: Clone> super::ToSql for insert_query::InsertQuery<T, V, M, RT, RL> {
    fn to_sql(&self, ctx: &mut super::SqlContext) -> String {
        let mut sql = format!("INSERT INTO {}", self.get_into().get_table_name());

        let maybe_cols = self.get_cols().as_ref();
        if maybe_cols.is_some() {
            let cols = maybe_cols.unwrap();

            if !cols.is_empty() {
                let cols_str: Vec<String> = cols.iter().map(|col| col.to_sql(ctx)).collect();
                sql = format!("{} ({})", sql, cols_str.connect(", "))
            }
        }

        sql = format!("{} {}", sql, self.get_values().to_sql(ctx));

        match self.get_returning() {
            &Some(ref select) => sql = format!("{} RETURNING {}", sql, select.to_sql(ctx)),
            &None => ()
        }

        sql
    }
}

impl<T: Clone, V: Clone+super::ToSql, M: Clone, RT: Clone, RL: Clone> super::QueryToSql for insert_query::InsertQuery<T, V, M, RT, RL> {}
