use from::{Table, RcTable, TableDef};

pub trait FromToSql {
    fn to_from_sql(&self) -> String;
}

impl FromToSql for TableDef {
    fn to_from_sql(&self) -> String {
        let name = self.get_table_name();
        match self.get_table_alias() {
            &Some(ref alias) => format!("{} AS {}", name, alias),
            &None => format!("{}", name),
        }
    }
}

impl FromToSql for RcTable {
    fn to_from_sql(&self) -> String {
        let name = self.get_table_name();
        match self.get_table_alias() {
            &Some(ref alias) => format!("{} AS {}", name, alias),
            &None => format!("{}", name),
        }
    }
}