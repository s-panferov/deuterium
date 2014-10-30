use sql::{SqlContext};
use from::{Table, RcTable, TableDef};

pub trait FromToSql {
    fn to_from_sql(&self, ctx: &mut SqlContext) -> String;
}

impl FromToSql for TableDef {
    fn to_from_sql(&self, _ctx: &mut SqlContext) -> String {
        let name = self.get_table_name();
        match self.get_table_alias() {
            &Some(ref alias) => format!("{} AS {}", name, alias),
            &None => format!("{}", name),
        }
    }
}

impl FromToSql for RcTable {
    fn to_from_sql(&self, _ctx: &mut SqlContext) -> String {
        let name = self.get_table_name();
        match self.get_table_alias() {
            &Some(ref alias) => format!("{} AS {}", name, alias),
            &None => format!("{}", name),
        }
    }
}