use insert_query::{
    InsertQuery, 
    Insert,
    InsertDefaultValues,
    InsertValues,
    InsertUntypedValues,
    InsertFromSelect,
};

use sql::{ToSql, QueryToSql};

impl<T: Clone, V: ToSql, M: Clone> ToSql for Insert<T, V, M> {
    fn to_sql(&self) -> String {
        match self {
            &InsertDefaultValues => {
                format!("DEFAULT VALUES")
            },
            &InsertValues(ref rows) => {
                let rows_str: Vec<String> = rows.iter().map(|row| { format!("({})", row.to_sql()) }).collect();
                format!("VALUES\n    {}", rows_str.connect(",\n    "))
            },
            &InsertUntypedValues(ref rows) => {
                let rows_str: Vec<String> = rows.iter().map(|row| {
                    let values_str: Vec<String> = row.iter().map(|v| v.to_sql()).collect();
                    format!("({})", values_str.connect(", "))
                }).collect();
                format!("VALUES\n    {}", rows_str.connect(",\n    "))    
            },
            &InsertFromSelect(ref select) => {
                select.to_sql()
            }
        }
    }
}

impl<T: Clone, V: Clone+ToSql, M: Clone> ToSql for InsertQuery<T, V, M> {
    fn to_sql(&self) -> String {
        let mut sql = format!("INSERT INTO {}", self.get_into().get_table_name());

        let maybe_cols = self.get_cols().as_ref();
        if maybe_cols.is_some() {
            let cols = maybe_cols.unwrap();

            if !cols.is_empty() {
                let cols_str: Vec<String> = cols.iter().map(|col| col.to_sql()).collect();
                sql = format!("{} ({})", sql, cols_str.connect(", "))
            }
        }

        format!("{} {}", sql, self.get_values().to_sql())
    }
}

impl<T: Clone, V: Clone+ToSql, M: Clone> QueryToSql for InsertQuery<T, V, M> {
    fn to_final_sql(&self) -> String {
        format!("{};", self.to_sql())
    }
}