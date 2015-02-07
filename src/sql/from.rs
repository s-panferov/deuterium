use super::super::from::{self, Table};

pub trait FromToSql {
    fn to_from_sql(&self, ctx: &mut super::SqlContext) -> String;
}

impl FromToSql for from::TableDef {
    fn to_from_sql(&self, _ctx: &mut super::SqlContext) -> String {
        let name = self.get_table_name();
        match self.get_table_alias() {
            &Some(ref alias) => format!("{} AS {}", name, alias),
            &None => format!("{}", name),
        }
    }
}

impl FromToSql for from::SharedTable {
    fn to_from_sql(&self, _ctx: &mut super::SqlContext) -> String {
        let name = self.get_table_name();
        match self.get_table_alias() {
            &Some(ref alias) => format!("{} AS {}", name, alias),
            &None => format!("{}", name),
        }
    }
}