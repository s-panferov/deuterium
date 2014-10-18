
use std::sync::Arc;
use to_sql::{FromToSql};
use select_query::{SelectQuery};

pub trait From: FromToSql { 
    fn upcast(&self) -> RcFrom;
}

pub type BoxedFrom = Box<From + Send + Sync>;
pub type RcFrom = Arc<BoxedFrom>;

#[deriving(Clone)]
pub struct TableDef {
    name: String,
    alias: Option<String>
}

impl TableDef {
    pub fn new(name: String) -> TableDef {
        TableDef { name: name, alias: None }
    }

    pub fn alias(&self, alias: String) -> TableDef {
        let mut table_def = self.clone();
        table_def.alias = Some(alias);
        table_def
    }

    pub fn get_name(&self) -> &String {
        &self.name
    }

    pub fn get_alias(&self) -> &Option<String> {
        &self.alias
    }
}

impl From for TableDef { 
    fn upcast(&self) -> RcFrom {
        Arc::new(box self.clone() as BoxedFrom)
    }
}

#[deriving(Clone)]
pub struct FromSelect<T,L> {
    pub select: SelectQuery<T,L>,
    pub alias: String 
}

impl<T: Clone, L: Clone> From for FromSelect<T,L> {
    fn upcast(&self) -> RcFrom {
        Arc::new(box self.clone() as BoxedFrom)
    }
}
